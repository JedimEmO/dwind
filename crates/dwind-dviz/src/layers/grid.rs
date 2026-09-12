//! Horizontal hairline gridlines at the y ticks. Solid, one step off the
//! surface, never dashed. Lines are keyed by tick label so they slide with
//! the axis.

use dominator::svg;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;

use super::{EXIT_MS, held, px, round, when_present, with_exit};
use crate::chart::{Layer, layer};

#[derive(Debug, Clone, PartialEq)]
struct GridLine {
    y: f64,
    x1: f64,
    x2: f64,
}

pub fn grid() -> Layer {
    layer(|ctx| {
        let frame = ctx.frame();
        let lines = frame
            .signal_ref(|f| {
                let baseline = f.plot.bottom();
                f.y_ticks()
                    .into_iter()
                    // The axis baseline already draws the bottom line.
                    .filter(|t| (t.position - baseline).abs() > 0.5)
                    .map(|t| {
                        (
                            t.label,
                            GridLine {
                                y: round(t.position),
                                x1: f.plot.x,
                                x2: f.plot.right(),
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .broadcast();
        let (shown, driver) = with_exit(
            lines
                .signal_ref(|v| v.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>())
                .dedupe_cloned(),
            EXIT_MS,
        );
        svg!("g", {
            .class("dviz-grid")
            .attr("aria-hidden", "true")
            .future(driver)
            .children_signal_vec(shown.keys().map(move |label| {
                let k = label.clone();
                let held_line = held(lines.signal_ref(move |v| v.iter().find(|(kk, _)| *kk == k).map(|(_, l)| l.clone())));
                let leaving = shown.leaving(label);
                when_present(held_line, move |one| svg!("g", {
                    .class(["dviz-tick", "dviz-enter"])
                    .class_signal("dviz-leave", leaving)
                    .attr_signal("transform", one.signal_ref(|l| format!("translate(0,{})", l.y)))
                    .child(svg!("line", {
                        .attr("stroke-width", "1")
                        .attr_signal("x1", one.signal_ref(|l| px(l.x1)))
                        .attr_signal("x2", one.signal_ref(|l| px(l.x2)))
                        .attr("y1", "0")
                        .attr("y2", "0")
                    }))
                }))
            }))
        })
    })
}
