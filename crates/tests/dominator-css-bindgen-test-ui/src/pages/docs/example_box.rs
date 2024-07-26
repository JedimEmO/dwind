use dominator::{Dom, events};
use futures_signals::signal::{Mutable, not, SignalExt};
use dwind_macros::{dwclass, dwclass_signal};
use dwind::prelude::*;

pub fn example_box(child: Dom, resizeable: bool) -> Dom {
    let width = Mutable::new(100.0f64);
    let dragging = Mutable::new(false);

    html!("div", {
        .dwclass!("relative grid m-t-10")
        .child(html!("div", {
            .dwclass!("rounded-lg bg-bunker-950")
            .dwclass!("border border-woodsmoke-800")
            .dwclass!("flex align-items-center p-5")
            .dwclass_signal!("pointer-events-none", dragging.signal())
            .style_signal("width", width.signal().map(|v| format!("{v}%")))
            .child(child)
        }))
        .child(html!("div", {
            .dwclass!("absolute w-full h-full flex align-items-center")
            .dwclass_signal!("pointer-events-auto", dragging.signal())
            .dwclass_signal!("pointer-events-none", not(dragging.signal()))
            .with_node!(element => {
                .event(clone!(dragging, width => move |event: events::MouseMove| {
                    if !dragging.get() {
                        return;
                    }

                    let bounding_rect = element.get_bounding_client_rect();
                    let offset_x = event.offset_x();
                    let pct = 100.0 * offset_x as f64 / bounding_rect.width();
                    width.set(50.0f64.max(100.0 * offset_x as f64 / bounding_rect.width()));
                }))
            })
            .child(html!("div", {
                .style_signal("right", width.signal().map(|v| format!("{}%", 97.0 - v)))
                .dwclass!("absolute bg-woodsmoke-600 rounded-md h-10 w-2 cursor-col-resize pointer-events-auto")
                .with_node!(element => {
                    .event(clone!(dragging => move |_: events::MouseDown| {
                        dragging.set(true);
                    }))
                    .event(clone!(dragging => move |event: events::MouseMove| {
                        event.stop_propagation();
                    }))
                    .global_event(clone!(dragging => move |_: events::MouseUp| {
                        dragging.set(false);
                    }))
                })
            }))
        }))
    })
}