use dominator::{Dom, svg};
use dwind_dviz_core::data::{Extent, Point};
use dwind_dviz_core::geom::{self, Curve};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

use crate::chart::{ChartProps, XDomain, YDomain, chart, layer};
use crate::layers::px;
use dwind_dviz_core::layout::Margins;
use dwind_dviz_core::stats;

/// A tiny trend line with no axes: the recent values in the muted ink, the
/// latest point in the accent. For stat tiles and table cells.
#[component(render_fn = sparkline)]
struct Sparkline {
    #[signal]
    #[default(vec![])]
    values: Vec<f64>,

    #[default(String::new())]
    label: String,

    #[default(32.0)]
    height: f64,

    /// Fix the y domain, otherwise the data extent.
    #[default(None)]
    y: Option<Extent<f64>>,
}

pub fn sparkline(props: SparklineProps) -> Dom {
    let SparklineProps {
        values,
        label,
        height,
        y,
        apply,
    } = props;

    let values = values.broadcast();
    let x_domain =
        values.signal_ref(|v| XDomain::Linear(Extent::new(0.0, (v.len().max(2) - 1) as f64)));
    let y_domain = values.signal_ref(move |v| {
        YDomain::Linear(
            y.unwrap_or_else(|| stats::extent(v.iter().copied()).unwrap_or(Extent::UNIT)),
        )
    });

    chart(ChartProps::new()
        .label(label)
        .height(height)
        .nice_y(false)
        .margins(Some(Margins::uniform(4.0)))
        .x_domain_signal(x_domain)
        .y_domain_signal(y_domain)
        .layers(vec![layer(move |ctx| {
            let frame = ctx.frame();
            let gradient = format!("url(#{})", ctx.gradient_id(0));
            let content = map_ref! {
                let f = frame.signal_cloned(),
                let v = values.signal_cloned() => {
                    let pts: Vec<Point> = v.iter().enumerate().map(|(i, y)| Point::new(f.x.map(i as f64), f.y.map(*y))).collect();
                    let d = geom::line(&pts, Curve::Linear);
                    let wash = geom::area(&pts, f.plot.bottom(), Curve::Linear);
                    let last = pts.iter().rev().find(|p| p.is_defined()).copied();
                    Some(svg!("g", {
                        .child(svg!("path", {
                            .class("dviz-sparkline-wash")
                            .attr("style", &format!("fill: {}", gradient))
                            .attr("d", &wash)
                        }))
                        .child(svg!("path", {
                            .class("dviz-sparkline")
                            .attr("d", &d)
                        }))
                        .apply_if(last.is_some(), |b| {
                            let p = last.unwrap();
                            b.child(svg!("circle", {
                                .class("dviz-sparkline-end")
                                .attr("cx", &px(p.x))
                                .attr("cy", &px(p.y))
                                .attr("r", "3")
                            }))
                        })
                    }))
                }
            };
            svg!("g", { .child_signal(content) })
        })])
        .apply(move |b| match apply {
            Some(a) => a(b),
            None => b,
        }))
}
