//! Axes: a hairline baseline plus recessive tick labels in muted ink.
//! Ticks are persistent nodes keyed by label, so a domain change slides
//! them into place and fades the ones that leave.

use dominator::{Dom, svg};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;

use super::{EXIT_MS, held, px, round, when_present, with_exit};
use crate::chart::{ChartContext, Layer, Tick, layer};

const TICK_LEN: f64 = 6.0;
const GAP: f64 = 4.0;

/// Bottom x axis with its ticks and labels.
pub fn axis_x() -> Layer {
    layer(|ctx| {
        let frame = ctx.frame();
        svg!("g", {
            .class(["dviz-axis", "dviz-axis-x"])
            .attr("aria-hidden", "true")
            .child(svg!("line", {
                .attr("stroke-width", "1")
                .attr_signal("x1", frame.signal_ref(|f| px(f.plot.x)))
                .attr_signal("x2", frame.signal_ref(|f| px(f.plot.right())))
                .attr_signal("y1", frame.signal_ref(|f| px(f.plot.bottom())))
                .attr_signal("y2", frame.signal_ref(|f| px(f.plot.bottom())))
            }))
            .child(keyed_ticks(ctx, Axis::X))
        })
    })
}

/// Left y axis with its ticks and labels.
pub fn axis_y() -> Layer {
    layer(|ctx| {
        svg!("g", {
            .class(["dviz-axis", "dviz-axis-y"])
            .attr("aria-hidden", "true")
            .child(keyed_ticks(ctx, Axis::Y))
        })
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

/// A tick's placement: the translate for its group.
#[derive(Debug, Clone, PartialEq)]
struct Placed {
    transform: String,
}

fn keyed_ticks(ctx: &Rc<ChartContext>, axis: Axis) -> Dom {
    let frame = ctx.frame();
    let ticks = frame
        .signal_ref(move |f| {
            let (list, base) = match axis {
                Axis::X => (f.x_ticks(), f.plot.bottom()),
                Axis::Y => (f.y_ticks(), f.plot.x),
            };
            list.into_iter()
                .map(|t: Tick| {
                    let transform = match axis {
                        Axis::X => format!("translate({},{})", round(t.position), round(base)),
                        Axis::Y => format!("translate({},{})", round(base), round(t.position)),
                    };
                    (t.label, Placed { transform })
                })
                .collect::<Vec<_>>()
        })
        .broadcast();
    let (shown, driver) = with_exit(
        ticks
            .signal_ref(|v| v.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>())
            .dedupe_cloned(),
        EXIT_MS,
    );
    svg!("g", {
        .future(driver)
        .children_signal_vec(shown.keys().map(move |label| {
            let k = label.clone();
            let held_tick = held(ticks.signal_ref(move |v| v.iter().find(|(kk, _)| *kk == k).map(|(_, p)| p.clone())));
            let leaving = shown.leaving(label.clone());
            when_present(held_tick, move |one| svg!("g", {
                .class(["dviz-tick", "dviz-enter"])
                .class_signal("dviz-leave", leaving)
                .attr_signal("transform", one.signal_ref(|p| p.transform.clone()))
                .apply(|b| match axis {
                    Axis::X => b
                        .child(svg!("line", {
                            .attr("y2", &px(TICK_LEN))
                        }))
                        .child(svg!("text", {
                            .attr("y", &px(TICK_LEN + GAP))
                            .attr("dy", "0.9em")
                            .attr("text-anchor", "middle")
                            .text(&label)
                        })),
                    Axis::Y => b.child(svg!("text", {
                        .attr("x", &px(-(TICK_LEN + GAP)))
                        .attr("dy", "0.32em")
                        .attr("text-anchor", "end")
                        .text(&label)
                    })),
                })
            }))
        }))
    })
}
