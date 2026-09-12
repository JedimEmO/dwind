//! Points: filled circles (>= 8px) with a 2px surface ring so they stay
//! legible where they overlap or cross a line. Each point is a persistent
//! node keyed by its index, so a moved point glides and a removed one
//! fades.

use dominator::svg;
use dwind_dviz_core::data::{Point, Series};
use dwind_dviz_core::format;
use futures_signals::signal::Signal;

use super::{MarkInput, hoverable, keyed_marks, px};
use crate::chart::{ChartContext, Layer, Tooltip, TooltipRow, layer};

/// Minimum hit-target radius in px (a 24px target), larger than the mark.
pub const HIT_RADIUS: f64 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointOptions {
    /// Visual radius of the fill in px. The spec minimum is 4 (8px marker).
    pub radius: f64,
    /// Ring width in the surface color.
    pub ring: f64,
}

impl Default for PointOptions {
    fn default() -> Self {
        Self {
            radius: 4.0,
            ring: 2.0,
        }
    }
}

/// A point's pixel position and data value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointMark {
    pub x: f64,
    pub y: f64,
    pub data: Point,
    pub x_label: usize,
}

pub fn points<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    points_with(series, PointOptions::default())
}

pub fn points_with<S>(series: S, opts: PointOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_marks(
            ctx,
            series,
            "dviz-points",
            true,
            |frame, all, si| {
                all[si]
                    .points
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| {
                        let (x, y) = (frame.x.map(p.x), frame.y.map(p.y));
                        if !x.is_finite() || !y.is_finite() {
                            return None;
                        }
                        Some((
                            i,
                            (
                                PointMark {
                                    x,
                                    y,
                                    data: *p,
                                    x_label: 0,
                                },
                                frame.x.describe(p.x),
                            ),
                        ))
                    })
                    .collect()
            },
            move |ctx: &std::rc::Rc<ChartContext>,
                  index: &usize,
                  input: MarkInput<(PointMark, String)>| {
                let color = ctx.color_for(&input.series_id);
                // Half the stroke lies outside the circle: grow r so the fill
                // keeps its radius and the ring sits around it.
                let r = opts.radius + opts.ring / 2.0;
                let label = input.series_label.clone();
                let tip = input.mark.signal_ref({
                    let color = color.clone();
                    move |(m, x_text)| Tooltip {
                        x: m.x,
                        y: m.y,
                        title: label.clone(),
                        rows: vec![
                            TooltipRow {
                                color: None,
                                label: "x".into(),
                                value: x_text.clone(),
                            },
                            TooltipRow {
                                color: Some(color.clone()),
                                label: "y".into(),
                                value: format::compact(m.data.y),
                            },
                        ],
                    }
                });
                let tooltip = ctx.tooltip().clone();
                svg!("g", {
                    .child(svg!("circle", {
                        .class("dviz-point")
                        .attr("data-index", &index.to_string())
                        .attr_signal("cx", input.mark.signal_ref(|(m, _)| px(m.x)))
                        .attr_signal("cy", input.mark.signal_ref(|(m, _)| px(m.y)))
                        .attr("r", &px(r))
                        .attr("stroke-width", &px(opts.ring))
                        .attr("style", &format!("fill: {color}; stroke: var(--dviz-surface)"))
                    }))
                    .child(svg!("circle", {
                        .class("dviz-hit")
                        .attr("tabindex", "0")
                        .attr_signal("cx", input.mark.signal_ref(|(m, _)| px(m.x)))
                        .attr_signal("cy", input.mark.signal_ref(|(m, _)| px(m.y)))
                        .attr("r", &px(HIT_RADIUS.max(r)))
                        .apply(|b| hoverable(b, tooltip, tip))
                    }))
                })
            },
        )
    })
}
