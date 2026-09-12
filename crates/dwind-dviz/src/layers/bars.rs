//! Bars: grouped or stacked columns growing from the baseline.
//!
//! Bars need a band x axis; each point's x is the category index. Marks
//! are at most 24px thick, rounded 4px at the data end and square at the
//! baseline, with a 2px surface gap between neighbours and between stacked
//! segments. Negative values grow downward from zero.
//!
//! Each bar is a persistent node keyed by category index, so a value
//! change tweens the path and a removed category fades out.

use dominator::svg;
use dwind_dviz_core::data::Series;
use dwind_dviz_core::format;
use dwind_dviz_core::geom::{self, RoundedEnd, StackOffset};
use futures_signals::signal::Signal;

use super::{MarkInput, hoverable, keyed_marks, px, stack_series};
use crate::chart::{ChartContext, Frame, Layer, Tooltip, TooltipRow, XScale, layer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarMode {
    /// Series side by side inside each band.
    #[default]
    Grouped,
    /// Series on top of each other, positives up and negatives down.
    Stacked,
    /// Stacked and normalised to 100% per category. Use a `0..=1` y domain.
    StackedExpand,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarOptions {
    pub mode: BarMode,
    /// Cap on bar thickness in px; leftover band space is air.
    pub max_thickness: f64,
    /// Corner radius at the data end.
    pub radius: f64,
    /// Surface gap between neighbours and stacked segments.
    pub gap: f64,
}

impl Default for BarOptions {
    fn default() -> Self {
        Self {
            mode: BarMode::Grouped,
            max_thickness: 24.0,
            radius: 4.0,
            gap: 2.0,
        }
    }
}

impl BarOptions {
    pub fn stacked() -> Self {
        Self {
            mode: BarMode::Stacked,
            ..Default::default()
        }
    }
}

/// One bar's pixel rectangle and the data behind it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub value: f64,
    /// Category index.
    pub index: usize,
    pub rounded: RoundedEnd,
    /// The category label, for tooltips.
    pub category_start: f64,
}

/// The rectangles for series `si` under `opts`, in the frame. Empty when
/// the x axis is not a band.
pub fn bar_geometry(frame: &Frame, all: &[Series], si: usize, opts: BarOptions) -> Vec<BarRect> {
    let XScale::Band(band) = &frame.x else {
        return vec![];
    };
    let Some(series) = all.get(si) else {
        return vec![];
    };
    let n = all.len().max(1) as f64;
    let bw = band.bandwidth();
    let baseline = frame.y.baseline();
    let mut out = Vec::with_capacity(series.points.len());

    match opts.mode {
        BarMode::Grouped => {
            let sub = bw / n;
            let thickness = (sub - opts.gap).min(opts.max_thickness).max(0.0);
            for p in &series.points {
                if !p.is_defined() {
                    continue;
                }
                let Some(x0) = band.map_index(p.x.round() as usize) else {
                    continue;
                };
                let top = frame.y.map(p.y);
                if !top.is_finite() {
                    continue;
                }
                let x = x0 + si as f64 * sub + (sub - thickness) / 2.0;
                let (y, h, rounded) = if p.y >= 0.0 {
                    (top, baseline - top, RoundedEnd::Top)
                } else {
                    (baseline, top - baseline, RoundedEnd::Bottom)
                };
                out.push(BarRect {
                    x,
                    y,
                    width: thickness,
                    height: h.max(0.0),
                    value: p.y,
                    index: p.x.round() as usize,
                    rounded,
                    category_start: x0,
                });
            }
        }
        BarMode::Stacked | BarMode::StackedExpand => {
            let offset = if opts.mode == BarMode::Stacked {
                StackOffset::Diverging
            } else {
                StackOffset::Expand
            };
            let stacked = stack_series(all, offset);
            let thickness = bw.min(opts.max_thickness);
            for (i, p) in series.points.iter().enumerate() {
                let Some(seg) = stacked[si].get(i) else {
                    continue;
                };
                if !p.is_defined() || seg.y0 == seg.y1 {
                    continue;
                }
                let Some(x0) = band.map_index(p.x.round() as usize) else {
                    continue;
                };
                let positive = seg.y1 >= seg.y0;
                // Outermost segment in this direction gets the rounded end.
                let outer = all
                    .iter()
                    .enumerate()
                    .filter(|(k, s)| {
                        s.points.get(i).is_some_and(|q| {
                            q.is_defined() && (q.y > 0.0) == positive && q.y != 0.0
                        }) && stacked[*k].get(i).is_some_and(|z| z.y0 != z.y1)
                    })
                    .map(|(k, _)| k)
                    .next_back();
                let is_outer = outer == Some(si);
                let is_first = seg.y0 == 0.0;
                let a = frame.y.map(seg.y0);
                let b = frame.y.map(seg.y1);
                if !a.is_finite() || !b.is_finite() {
                    continue;
                }
                let (mut y, mut h) = (a.min(b), (a - b).abs());
                // Surface gap on the baseline side of every non-first segment.
                if !is_first {
                    if positive {
                        h -= opts.gap;
                    } else {
                        y += opts.gap;
                        h -= opts.gap;
                    }
                }
                out.push(BarRect {
                    x: x0 + (bw - thickness) / 2.0,
                    y,
                    width: thickness,
                    height: h.max(0.0),
                    value: p.y,
                    index: p.x.round() as usize,
                    rounded: match (is_outer, positive) {
                        (false, _) => RoundedEnd::None,
                        (true, true) => RoundedEnd::Top,
                        (true, false) => RoundedEnd::Bottom,
                    },
                    category_start: x0,
                });
            }
        }
    }
    out
}

pub fn bars<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    bars_with(series, BarOptions::default())
}

pub fn bars_with<S>(series: S, opts: BarOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_marks(
            ctx,
            series,
            "dviz-bars",
            true,
            move |frame, all, si| {
                let categories = match &frame.x {
                    XScale::Band(b) => b.categories().to_vec(),
                    _ => vec![],
                };
                bar_geometry(frame, all, si, opts)
                    .into_iter()
                    .map(|r| {
                        let category = categories.get(r.index).cloned().unwrap_or_default();
                        (r.index, (r, category))
                    })
                    .collect()
            },
            move |ctx: &std::rc::Rc<ChartContext>,
                  index: &usize,
                  input: MarkInput<(BarRect, String)>| {
                let color = ctx.color_for(&input.series_id);
                let label = input.series_label.clone();
                let tip = input.mark.signal_ref({
                    let color = color.clone();
                    move |(r, category)| Tooltip {
                        x: r.x + r.width / 2.0,
                        y: r.y,
                        title: category.clone(),
                        rows: vec![TooltipRow {
                            color: Some(color.clone()),
                            label: label.clone(),
                            value: format::compact(r.value),
                        }],
                    }
                });
                let tooltip = ctx.tooltip().clone();
                svg!("path", {
                    .class("dviz-bar")
                    .attr("tabindex", "0")
                    .attr("data-index", &index.to_string())
                    .attr_signal("data-value", input.mark.signal_ref(|(r, _)| px(r.value)))
                    .attr_signal("style", input.mark.signal_ref(move |(r, _)| {
                        let origin = if r.rounded == RoundedEnd::Bottom { "top" } else { "bottom" };
                        format!("fill: {color}; transform-origin: {origin}")
                    }))
                    .attr_signal("d", input.mark.signal_ref(move |(r, _)| {
                        geom::bar(r.x, r.y, r.width, r.height, opts.radius, r.rounded)
                    }))
                    .apply(|b| hoverable(b, tooltip, tip))
                })
            },
        )
    })
}

/// Column totals for stacked bars, or the single value for grouped bars:
/// what a value label on the cap should show.
pub fn column_totals(all: &[Series]) -> Vec<(usize, f64)> {
    let n = all.iter().map(|s| s.points.len()).max().unwrap_or(0);
    (0..n)
        .filter_map(|i| {
            let index = all
                .iter()
                .find_map(|s| s.points.get(i))
                .map(|p| p.x.round() as usize)?;
            let total: f64 = all
                .iter()
                .filter_map(|s| s.points.get(i))
                .filter(|p| p.is_defined())
                .map(|p| p.y)
                .sum();
            Some((index, total))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{XDomain, YDomain};
    use dwind_dviz_core::data::{Extent, Point};
    use dwind_dviz_core::layout::{Margins, Rect};

    fn frame(all: &[Series], nice: bool) -> Frame {
        let y = dwind_dviz_core::data::y_extent_of(all)
            .unwrap()
            .include_zero();
        Frame::compute(
            Rect::from_size(300.0, 200.0),
            &XDomain::band(["a", "b", "c"]),
            &YDomain::Linear(y),
            nice,
            Some(Margins::uniform(0.0)),
        )
    }

    fn series(id: &str, ys: &[f64]) -> Series {
        Series::new(
            id,
            id,
            ys.iter()
                .enumerate()
                .map(|(i, y)| Point::new(i as f64, *y))
                .collect(),
        )
    }

    #[test]
    fn grouped_bars_sit_side_by_side_from_the_baseline() {
        let all = vec![
            series("a", &[10.0, -5.0, 0.0]),
            series("b", &[20.0, 5.0, 8.0]),
        ];
        let f = frame(&all, false);
        let a = bar_geometry(&f, &all, 0, BarOptions::default());
        let b = bar_geometry(&f, &all, 1, BarOptions::default());
        assert_eq!(a.len(), 3);
        assert!(a[0].x < b[0].x, "series a left of b");
        assert!(a[0].width <= 24.0);
        assert!(
            (a[0].y + a[0].height - f.y.baseline()).abs() < 1e-9,
            "positive bar ends at the baseline"
        );
        assert!(
            (a[1].y - f.y.baseline()).abs() < 1e-9,
            "negative bar starts at the baseline"
        );
        assert_eq!(a[1].rounded, RoundedEnd::Bottom);
        assert_eq!(a[2].height, 0.0);
    }

    #[test]
    fn stacked_bars_tile_with_a_gap_and_round_only_the_outer_end() {
        let all = vec![
            series("a", &[10.0]),
            series("b", &[20.0]),
            series("c", &[5.0]),
        ];
        let f = frame(&all, false);
        let o = BarOptions::stacked();
        let a = bar_geometry(&f, &all, 0, o)[0];
        let b = bar_geometry(&f, &all, 1, o)[0];
        let c = bar_geometry(&f, &all, 2, o)[0];
        assert_eq!(a.rounded, RoundedEnd::None);
        assert_eq!(b.rounded, RoundedEnd::None);
        assert_eq!(c.rounded, RoundedEnd::Top);
        assert!((a.y + a.height - f.y.baseline()).abs() < 1e-9);
        assert!(b.y + b.height <= a.y - o.gap + 1e-9, "gap between a and b");
        assert!(
            (c.y - f.y.map(35.0)).abs() < 1e-9,
            "top of the stack is the total"
        );
        assert_eq!(a.x, b.x);
    }

    #[test]
    fn expand_stacks_to_one() {
        let all = vec![series("a", &[1.0, 3.0]), series("b", &[3.0, 1.0])];
        let f = Frame::compute(
            Rect::from_size(300.0, 200.0),
            &XDomain::band(["a", "b"]),
            &YDomain::Linear(Extent::UNIT),
            false,
            Some(Margins::uniform(0.0)),
        );
        let o = BarOptions {
            mode: BarMode::StackedExpand,
            ..Default::default()
        };
        let top = bar_geometry(&f, &all, 1, o)[0];
        assert!((top.y - f.y.map(1.0)).abs() < 1e-9);
        assert_eq!(column_totals(&all), vec![(0, 4.0), (1, 4.0)]);
    }

    #[test]
    fn no_band_no_bars() {
        let all = vec![series("a", &[1.0])];
        let f = Frame::compute(
            Rect::from_size(300.0, 200.0),
            &XDomain::Linear(Extent::UNIT),
            &YDomain::Linear(Extent::UNIT),
            false,
            None,
        );
        assert!(bar_geometry(&f, &all, 0, BarOptions::default()).is_empty());
    }
}
