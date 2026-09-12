//! Reference lines, bands and event markers: context the reader needs to
//! judge the data, drawn in ink so they never impersonate a series.

use dominator::{Dom, svg};
use dwind_dviz_core::data::Extent;
use futures_signals::map_ref;
use futures_signals::signal::Signal;

use super::px;
use crate::chart::{Frame, Layer, layer};

/// A horizontal reference line at a y value, labelled at its right end.
pub fn reference_y<V, L>(value: V, label: L) -> Layer
where
    V: Signal<Item = f64> + 'static,
    L: Signal<Item = String> + 'static,
{
    layer(move |ctx| {
        let frame = ctx.frame();
        let content = map_ref! {
            let f = frame.signal_cloned(),
            let v = value,
            let l = label => Some(reference_y_dom(f, *v, l))
        };
        svg!("g", {
            .class("dviz-annotations")
            .attr("aria-hidden", "true")
            .child_signal(content)
        })
    })
}

fn reference_y_dom(f: &Frame, v: f64, label: &str) -> Dom {
    let y = f.y.map(v);
    if !y.is_finite() || y < f.plot.y || y > f.plot.bottom() {
        return svg!("g", {});
    }
    svg!("g", {
        .child(svg!("line", {
            .class("dviz-reference")
            .attr("x1", &px(f.plot.x))
            .attr("x2", &px(f.plot.right()))
            .attr("y1", &px(y))
            .attr("y2", &px(y))
        }))
        .child(svg!("text", {
            .class("dviz-label")
            .attr("x", &px(f.plot.x + 4.0))
            .attr("y", &px(y - 4.0))
            .text(label)
        }))
    })
}

/// A vertical marker at an x value (a deploy, an incident), labelled at
/// the top.
pub fn reference_x<V, L>(value: V, label: L) -> Layer
where
    V: Signal<Item = f64> + 'static,
    L: Signal<Item = String> + 'static,
{
    layer(move |ctx| {
        let frame = ctx.frame();
        let content = map_ref! {
            let f = frame.signal_cloned(),
            let v = value,
            let l = label => Some(reference_x_dom(f, *v, l))
        };
        svg!("g", {
            .class("dviz-annotations")
            .attr("aria-hidden", "true")
            .child_signal(content)
        })
    })
}

fn reference_x_dom(f: &Frame, v: f64, label: &str) -> Dom {
    let x = f.x.map(v);
    if !x.is_finite() || x < f.plot.x || x > f.plot.right() {
        return svg!("g", {});
    }
    svg!("g", {
        .child(svg!("line", {
            .class("dviz-reference")
            .attr("x1", &px(x))
            .attr("x2", &px(x))
            .attr("y1", &px(f.plot.y))
            .attr("y2", &px(f.plot.bottom()))
        }))
        .child(svg!("text", {
            .class("dviz-label")
            .attr("x", &px(x + 4.0))
            .attr("y", &px(f.plot.y))
            .attr("dy", "0.9em")
            .text(label)
        }))
    })
}

/// A shaded horizontal band between two y values (a target range, an SLO).
pub fn band_y<E, L>(extent: E, label: L) -> Layer
where
    E: Signal<Item = Extent<f64>> + 'static,
    L: Signal<Item = String> + 'static,
{
    layer(move |ctx| {
        let frame = ctx.frame();
        let content = map_ref! {
            let f = frame.signal_cloned(),
            let e = extent,
            let l = label => Some(band_y_dom(f, *e, l))
        };
        svg!("g", {
            .class("dviz-annotations")
            .attr("aria-hidden", "true")
            .child_signal(content)
        })
    })
}

fn band_y_dom(f: &Frame, e: Extent<f64>, label: &str) -> Dom {
    let top = f.y.map(e.max).max(f.plot.y);
    let bottom = f.y.map(e.min).min(f.plot.bottom());
    if !top.is_finite() || !bottom.is_finite() || bottom <= top {
        return svg!("g", {});
    }
    svg!("g", {
        .child(svg!("rect", {
            .class("dviz-band")
            .attr("x", &px(f.plot.x))
            .attr("y", &px(top))
            .attr("width", &px(f.plot.width))
            .attr("height", &px(bottom - top))
        }))
        .child(svg!("text", {
            .class("dviz-label")
            .attr("x", &px(f.plot.x + 4.0))
            .attr("y", &px(top))
            .attr("dy", "0.9em")
            .text(label)
        }))
    })
}
