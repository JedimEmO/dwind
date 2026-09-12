//! Heatmap cells on two band axes, colored by a sequential or diverging
//! scale computed for the current theme mode. Each cell is a persistent
//! node keyed by its grid position, so a value change tweens its color
//! and a resize glides the grid.

use dominator::svg;
use dwind_dviz_core::data::Extent;
use dwind_dviz_core::format;
use dwind_dviz_core::palette::{Mode, defaults};
use dwind_dviz_core::scale::{DivergingScale, SequentialScale};
use dwind_dviz_core::stats;
use futures_signals::map_ref;
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;

use super::{EXIT_MS, held, hoverable, px, when_present, with_exit};
use crate::chart::{Frame, Layer, Tooltip, TooltipRow, XScale, layer};
use crate::theme;

/// One heatmap cell: category indices on both axes and the value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub value: f64,
}

impl Cell {
    pub const fn new(x: usize, y: usize, value: f64) -> Self {
        Self { x, y, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellScale {
    /// One hue, light to dark. Magnitude.
    Sequential,
    /// Two hues around a neutral midpoint. Polarity.
    Diverging { midpoint: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellOptions {
    pub scale: CellScale,
    /// Fix the value domain; otherwise it is the data extent.
    pub domain: Option<Extent<f64>>,
    /// Surface gap between cells.
    pub gap: f64,
    pub radius: f64,
}

impl Default for CellOptions {
    fn default() -> Self {
        Self {
            scale: CellScale::Sequential,
            domain: None,
            gap: 2.0,
            radius: 2.0,
        }
    }
}

/// The color for `value` under `opts` in `mode`, as hex.
pub fn cell_color(value: f64, domain: Extent<f64>, opts: CellOptions, mode: Mode) -> String {
    match opts.scale {
        CellScale::Sequential => SequentialScale::new(domain, defaults::sequential(mode))
            .map(value)
            .to_hex(),
        CellScale::Diverging { midpoint } => {
            DivergingScale::symmetric(domain, midpoint, defaults::diverging(mode))
                .map(value)
                .to_hex()
        }
    }
}

/// A cell's rectangle, color, and tooltip text.
#[derive(Debug, Clone, PartialEq)]
pub struct CellMark {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill: String,
    pub title: String,
    pub value: f64,
}

/// Every cell's geometry in the frame, keyed by grid position.
pub fn cell_geometry(
    f: &Frame,
    data: &[Cell],
    opts: CellOptions,
    mode: Mode,
) -> Vec<((usize, usize), CellMark)> {
    let (XScale::Band(xb), Some(yb)) = (&f.x, f.y.band()) else {
        return vec![];
    };
    let domain = opts
        .domain
        .or_else(|| stats::extent(data.iter().map(|c| c.value)))
        .unwrap_or(Extent::UNIT);
    let w = (xb.bandwidth() - opts.gap).max(0.0);
    let h = (yb.bandwidth() - opts.gap).max(0.0);
    data.iter()
        .filter_map(|c| {
            if !c.value.is_finite() {
                return None;
            }
            let x = xb.map_index(c.x)? + opts.gap / 2.0;
            let y = yb.map_index(c.y)? + opts.gap / 2.0;
            Some((
                (c.x, c.y),
                CellMark {
                    x,
                    y,
                    width: w,
                    height: h,
                    fill: cell_color(c.value, domain, opts, mode),
                    title: format!(
                        "{} · {}",
                        yb.categories().get(c.y).cloned().unwrap_or_default(),
                        xb.categories().get(c.x).cloned().unwrap_or_default()
                    ),
                    value: c.value,
                },
            ))
        })
        .collect()
}

pub fn cells<S>(cells: S, opts: CellOptions) -> Layer
where
    S: Signal<Item = Vec<Cell>> + 'static,
{
    layer(move |ctx| {
        let frame = ctx.frame();
        let marks = map_ref! {
            let f = frame.signal_cloned(),
            let data = cells,
            let mode = theme::mode_signal() => cell_geometry(f, data, opts, *mode)
        }
        .broadcast();
        let (shown, driver) = with_exit(
            marks
                .signal_ref(|v| v.iter().map(|(k, _)| *k).collect::<Vec<_>>())
                .dedupe_cloned(),
            EXIT_MS,
        );
        let tooltip = ctx.tooltip().clone();
        let clip = ctx.clip_url();
        svg!("g", {
            .class("dviz-cells")
            .attr("clip-path", &clip)
            .future(driver)
            .children_signal_vec(shown.keys().map(move |key| {
                let held_mark = held(marks.signal_ref(move |v| v.iter().find(|(k, _)| *k == key).map(|(_, m)| m.clone())));
                let tooltip = tooltip.clone();
                svg!("g", {
                    .class("dviz-enter")
                    .class_signal("dviz-leave", shown.leaving(key))
                    .child(when_present(held_mark, move |one| {
                        let tip = one.signal_ref(|m| Tooltip {
                            x: m.x + m.width / 2.0,
                            y: m.y,
                            title: m.title.clone(),
                            rows: vec![TooltipRow { color: Some(m.fill.clone()), label: "value".into(), value: format::compact(m.value) }],
                        });
                        svg!("rect", {
                            .class("dviz-cell")
                            .attr("tabindex", "0")
                            .attr("data-x", &key.0.to_string())
                            .attr("data-y", &key.1.to_string())
                            .attr_signal("x", one.signal_ref(|m| px(m.x)))
                            .attr_signal("y", one.signal_ref(|m| px(m.y)))
                            .attr_signal("width", one.signal_ref(|m| px(m.width)))
                            .attr_signal("height", one.signal_ref(|m| px(m.height)))
                            .attr("rx", &px(opts.radius))
                            .attr_signal("fill", one.signal_ref(|m| m.fill.clone()))
                            .apply(|b| hoverable(b, tooltip, tip))
                        })
                    }))
                })
            }))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_follow_the_ramp_direction_per_mode() {
        let d = Extent::new(0.0, 10.0);
        let lo = cell_color(0.0, d, CellOptions::default(), Mode::Light);
        let hi = cell_color(10.0, d, CellOptions::default(), Mode::Light);
        assert_eq!(lo, "#cde2fb");
        assert_eq!(hi, "#0d366b");
        assert_eq!(
            cell_color(0.0, d, CellOptions::default(), Mode::Dark),
            "#0d366b",
            "dark flips the anchor"
        );
        let mid = cell_color(
            0.0,
            Extent::new(-5.0, 5.0),
            CellOptions {
                scale: CellScale::Diverging { midpoint: 0.0 },
                ..Default::default()
            },
            Mode::Light,
        );
        assert_eq!(mid, "#ececef");
    }
}
