//! The line mark: one `<path>` per series, 2px, round joins, with an end
//! marker on the newest point.
//!
//! Each series is a persistent node whose `d` is a signal derived from the
//! frame and the data, so a live series updates one attribute per change,
//! a same-length update tweens, and a removed series fades out.

use dominator::{Dom, svg};
use dwind_dviz_core::data::{Point, Series};
use dwind_dviz_core::geom::{self, Curve};
use futures_signals::signal::Signal;
use std::rc::Rc;

use super::{MarkInput, keyed_marks, px};
use crate::chart::{ChartContext, Frame, Layer, layer};

/// When a line carries a wash beneath it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Wash {
    #[default]
    Never,
    /// Only while it is the lone series: reads well alone, noisy past two.
    WhenSingle,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineOptions {
    pub curve: Curve,
    /// Stroke width in px. The spec is 2.
    pub width: f64,
    /// An 8px marker with a surface ring on the last defined point: the
    /// current value, which is what a live reader looks for first.
    pub end_marker: bool,
    /// When to draw a faint wash of the series hue under the line.
    pub wash: Wash,
    /// A pulsing halo on the end marker: this series is being updated live.
    pub live: bool,
}

impl Default for LineOptions {
    fn default() -> Self {
        Self {
            curve: Curve::Linear,
            width: 2.0,
            end_marker: true,
            wash: Wash::Never,
            live: false,
        }
    }
}

/// Lines for every series in the signal, with default options.
pub fn line<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    line_with(series, LineOptions::default())
}

/// Lines for every series in the signal.
pub fn line_with<S>(series: S, options: LineOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        keyed_marks(
            ctx,
            series,
            "dviz-lines",
            false,
            move |frame, all, si| {
                let washed = match options.wash {
                    Wash::Never => false,
                    Wash::WhenSingle => all.len() == 1,
                    Wash::Always => true,
                };
                vec![(
                    (),
                    LineGeometry::new(frame, &all[si].points, options.curve, washed),
                )]
            },
            move |ctx: &Rc<ChartContext>, _: &(), input: MarkInput<LineGeometry>| {
                series_group(ctx, &input.series_id, input.mark, options)
            },
        )
    })
}

/// The pixel-space paths and end point of one series.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineGeometry {
    pub line: String,
    pub wash: String,
    /// Last defined point inside the plot, in px.
    pub end: Option<Point>,
}

impl LineGeometry {
    pub fn new(frame: &Frame, points: &[Point], curve: Curve, washed: bool) -> Self {
        let mapped: Vec<Point> = points
            .iter()
            .map(|p| Point::new(frame.x.map(p.x), frame.y.map(p.y)))
            .collect();
        let right = frame.plot.right() + 0.5;
        let left = frame.plot.x - 0.5;
        let end = mapped
            .iter()
            .rev()
            .find(|p| p.is_defined() && p.x <= right && p.x >= left)
            .copied();
        Self {
            line: geom::line(&mapped, curve),
            wash: if washed {
                geom::area(&mapped, frame.y.baseline(), curve)
            } else {
                String::new()
            },
            end,
        }
    }
}

fn series_group(
    ctx: &Rc<ChartContext>,
    id: &str,
    geometry: futures_signals::signal::Broadcaster<impl Signal<Item = LineGeometry> + 'static>,
    options: LineOptions,
) -> Dom {
    let color = ctx.color_for(id);
    let gradient = ctx.gradient_for(id);
    let clip = ctx.clip_url();
    let live = options.live;
    // The end marker persists and glides; it hides (rather than vanishes)
    // when the series has no visible end.
    let end_x = geometry.signal_ref(|g| g.end.map_or("0".to_string(), |p| px(p.x)));
    let end_y = geometry.signal_ref(|g| g.end.map_or("0".to_string(), |p| px(p.y)));
    let end_visible = geometry.signal_ref(|g| if g.end.is_some() { "1" } else { "0" });
    svg!("g", {
        .attr("stroke-linejoin", "round")
        .attr("stroke-linecap", "round")
        .attr("stroke-width", &format!("{}", options.width))
        .attr("style", &format!("color: {color}"))
        .apply_if(options.wash != Wash::Never, |b| b.child(svg!("path", {
            .class("dviz-wash")
            .attr("clip-path", &clip)
            .attr("style", &format!("fill: {gradient}"))
            .attr_signal("d", geometry.signal_ref(|g| g.wash.clone()))
        })))
        .child(svg!("path", {
            .class("dviz-line")
            .attr("data-series", id)
            .attr("clip-path", &clip)
            .attr("fill", "none")
            .attr("pathLength", "1")
            .attr("style", &format!("stroke: {color}"))
            .attr_signal("d", geometry.signal_ref(|g| g.line.clone()))
        }))
        .apply_if(options.end_marker, |b| b.child(svg!("g", {
            .class("dviz-end")
            .attr_signal("opacity", end_visible)
            .apply_if(live, |b| b.child(svg!("circle", {
                .class("dviz-halo")
                .attr_signal("cx", geometry.signal_ref(|g| g.end.map_or("0".to_string(), |p| px(p.x))))
                .attr_signal("cy", geometry.signal_ref(|g| g.end.map_or("0".to_string(), |p| px(p.y))))
                .attr("r", "4")
            })))
            .child(svg!("circle", {
                .class("dviz-end-marker")
                .attr_signal("cx", end_x)
                .attr_signal("cy", end_y)
                .attr("r", "4")
                .attr("style", &format!("fill: {color}"))
            }))
        })))
    })
}

/// Maps data points through the frame's scales and builds the path.
pub fn path_for(frame: &Frame, points: &[Point], curve: Curve) -> String {
    let mapped: Vec<Point> = points
        .iter()
        .map(|p| Point::new(frame.x.map(p.x), frame.y.map(p.y)))
        .collect();
    geom::line(&mapped, curve)
}
