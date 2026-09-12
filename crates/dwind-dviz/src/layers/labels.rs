//! Selective direct labels. Never a number on every point: series names at
//! line ends, values on bar caps when there are few enough bars.
//!
//! Text wears the ink tokens, never the series color; the mark beside it
//! carries identity.

use dominator::svg;
use dwind_dviz_core::data::Series;
use dwind_dviz_core::format;
use futures_signals::signal::Signal;

use super::bars::{BarMode, BarOptions, bar_geometry, column_totals};
use super::{keyed_series_groups, px};
use crate::chart::{ChartContext, Layer, XScale, layer};

/// Series names at the right end of each line. Best with four or fewer
/// series; past that, converging labels turn to noise and the legend plus
/// tooltip should carry identity instead.
pub fn line_end_labels<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_series_groups(
            ctx,
            series,
            "dviz-labels dviz-labels-end",
            false,
            move |frame, all, si, _: &ChartContext| {
                let s = &all[si];
                let last = s.points.iter().rev().find(|p| {
                    p.is_defined() && frame.x.map(p.x).is_finite() && frame.y.map(p.y).is_finite()
                });
                match last {
                    Some(p) => svg!("text", {
                        .class("dviz-label")
                        .attr("x", &px(frame.x.map(p.x) + 10.0))
                        .attr("y", &px(frame.y.map(p.y)))
                        .attr("dy", "0.32em")
                        .text(&s.label)
                    }),
                    None => svg!("g", {}),
                }
            },
        )
    })
}

/// Bars per series above which cap labels are dropped (the axis and the
/// tooltip carry the values instead).
pub const MAX_LABELLED_BARS: usize = 16;

/// Values on the caps: one per bar for grouped bars, the column total for
/// stacked bars. Uses the compact figure format.
pub fn bar_value_labels<S>(series: S, opts: BarOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_series_groups(
            ctx,
            series,
            "dviz-labels dviz-labels-bar",
            false,
            move |frame, all, si, _: &ChartContext| {
                let too_many = all.iter().any(|s| s.points.len() > MAX_LABELLED_BARS);
                let XScale::Band(band) = &frame.x else {
                    return svg!("g", {});
                };
                if too_many {
                    return svg!("g", {});
                }
                let texts: Vec<_> = match opts.mode {
                BarMode::Grouped => bar_geometry(frame, all, si, opts)
                    .into_iter()
                    .map(|r| {
                        let above = r.value >= 0.0;
                        svg!("text", {
                            .class("dviz-label")
                            .attr("x", &px(r.x + r.width / 2.0))
                            .attr("y", &px(if above { r.y - 4.0 } else { r.y + r.height + 4.0 }))
                            .attr("dy", if above { "0" } else { "0.9em" })
                            .attr("text-anchor", "middle")
                            .text(&format::compact(r.value))
                        })
                    })
                    .collect(),
                BarMode::Stacked | BarMode::StackedExpand => {
                    // Draw totals once, from the first series' group.
                    if si != 0 {
                        return svg!("g", {});
                    }
                    column_totals(all)
                        .into_iter()
                        .filter_map(|(index, total)| {
                            let x0 = band.map_index(index)?;
                            let top = frame.y.map(if opts.mode == BarMode::StackedExpand { 1.0 } else { total });
                            if !top.is_finite() {
                                return None;
                            }
                            let label = format::compact(total);
                            Some(svg!("text", {
                                .class("dviz-label")
                                .attr("x", &px(x0 + band.bandwidth() / 2.0))
                                .attr("y", &px(top - 4.0))
                                .attr("text-anchor", "middle")
                                .text(&label)
                            }))
                        })
                        .collect()
                }
            };
                svg!("g", { .children(texts) })
            },
        )
    })
}
