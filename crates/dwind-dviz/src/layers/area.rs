//! Areas: a wash of the series hue under its line, or stacked bands. One
//! persistent fill and line path per series with signal-driven `d`, so a
//! value change tweens when the point count is unchanged.

use dominator::svg;
use dwind_dviz_core::data::{Point, Series};
use dwind_dviz_core::geom::{self, Curve, StackOffset};
use futures_signals::signal::Signal;

use super::{MarkInput, keyed_marks, stack_series};
use crate::chart::{ChartContext, Frame, Layer, layer};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AreaOptions {
    pub curve: Curve,
    /// Stack the series (all must share the same x order).
    pub stacked: bool,
    /// Fill opacity: a wash, never a block. Stacked bands read better a
    /// little stronger since they never overlap.
    pub fill_opacity: f64,
    /// Draw the 2px line on the top edge.
    pub line: bool,
}

impl Default for AreaOptions {
    fn default() -> Self {
        Self {
            curve: Curve::Linear,
            stacked: false,
            fill_opacity: 0.12,
            line: true,
        }
    }
}

impl AreaOptions {
    pub fn stacked() -> Self {
        Self {
            stacked: true,
            fill_opacity: 0.35,
            ..Default::default()
        }
    }
}

pub fn area<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    area_with(series, AreaOptions::default())
}

pub fn area_with<S>(series: S, opts: AreaOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_marks(
            ctx,
            series,
            "dviz-areas",
            true,
            move |frame, all, si| vec![((), area_paths(frame, all, si, opts))],
            move |ctx: &std::rc::Rc<ChartContext>, _: &(), input: MarkInput<(String, String)>| {
                let color = ctx.color_for(&input.series_id);
                // Stacked bands tile, so they keep a flat fill; a lone area
                // gets the vertical fade in vivid mode.
                let paint = if opts.stacked {
                    format!("fill: {color}; fill-opacity: {}", opts.fill_opacity)
                } else {
                    format!(
                        "fill: {}; fill-opacity: {}",
                        ctx.gradient_for(&input.series_id),
                        1.0f64.min(opts.fill_opacity * 4.0)
                    )
                };
                svg!("g", {
                    .child(svg!("path", {
                        .class("dviz-area")
                        .apply_if(!opts.stacked, |b| b.class("dviz-area-solo"))
                        .attr("style", &paint)
                        .attr_signal("d", input.mark.signal_ref(|(fill, _)| fill.clone()))
                    }))
                    .apply_if(opts.line, |b| b.child(svg!("path", {
                        .class("dviz-line")
                        .attr("fill", "none")
                        .attr("stroke-width", "2")
                        .attr("stroke-linejoin", "round")
                        .attr("stroke-linecap", "round")
                        .attr("pathLength", "1")
                        .attr("style", &format!("stroke: {color}; color: {color}"))
                        .attr_signal("d", input.mark.signal_ref(|(_, top)| top.clone()))
                    })))
                })
            },
        )
    })
}

/// `(fill path, top line path)` for series `si`.
pub fn area_paths(frame: &Frame, all: &[Series], si: usize, opts: AreaOptions) -> (String, String) {
    let series = &all[si];
    if opts.stacked {
        let stacked = stack_series(all, StackOffset::None);
        let top: Vec<Point> = series
            .points
            .iter()
            .zip(&stacked[si])
            .map(|(p, s)| {
                Point::new(
                    frame.x.map(p.x),
                    if p.is_defined() {
                        frame.y.map(s.y1)
                    } else {
                        f64::NAN
                    },
                )
            })
            .collect();
        let bottom: Vec<Point> = series
            .points
            .iter()
            .zip(&stacked[si])
            .map(|(p, s)| {
                Point::new(
                    frame.x.map(p.x),
                    if p.is_defined() {
                        frame.y.map(s.y0)
                    } else {
                        f64::NAN
                    },
                )
            })
            .collect();
        (
            geom::band(&top, &bottom, opts.curve),
            geom::line(&top, opts.curve),
        )
    } else {
        let top: Vec<Point> = series
            .points
            .iter()
            .map(|p| Point::new(frame.x.map(p.x), frame.y.map(p.y)))
            .collect();
        (
            geom::area(&top, frame.y.baseline(), opts.curve),
            geom::line(&top, opts.curve),
        )
    }
}
