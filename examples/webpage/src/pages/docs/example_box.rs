use dominator::{events, Dom};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use futures_signals::signal::{not, Mutable, SignalExt};

/// A live preview frame for documentation examples.
///
/// Renders the example inside a bordered panel with a mono header bar;
/// when `resizeable` is set, a drag handle lets the reader shrink the
/// preview to test responsive behavior.
pub fn example_box(child: Dom, resizeable: bool) -> Dom {
    let width = Mutable::new(100.0f64);
    let dragging = Mutable::new(false);

    html!("div", {
        .dwclass!("m-t-6 rounded-lg border border-woodsmoke-800 overflow-hidden w-full")
        // header bar
        .child(html!("div", {
            .dwclass!("flex flex-row justify-between align-items-center h-8 p-l-4 p-r-4 border-b border-woodsmoke-800")
            .style("background", "rgba(18, 18, 21, 0.7)")
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-xs text-woodsmoke-500 select-none")
                .text("// preview")
            }))
            .apply_if(resizeable, |b| {
                b.child(html!("span", {
                    .class("font-code")
                    .dwclass!("text-xs text-woodsmoke-600 select-none")
                    .text("drag handle to resize ↔")
                }))
            })
        }))
        // body
        .child(html!("div", {
            .dwclass!("relative grid p-5")
            .style("background", "rgba(2, 2, 3, 0.5)")
            .child(html!("div", {
                .dwclass!("flex justify-center align-items-center")
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
                        width.set(30.0f64.max(pct));
                    }))
                })
                .apply_if(resizeable, |b| {
                    b.child(html!("div", {
                        .attr("role", "separator")
                        .attr("aria-label", "Resize preview")
                        // keep the handle inside the clipped frame even at 100% width
                        .style_signal("right", width.signal().map(|v| format!("max(0.4rem, {}%)", 97.0 - v)))
                        .dwclass!("absolute rounded-md h-10 w-2 cursor-col-resize pointer-events-auto transition-colors")
                        .dwclass!("bg-candlelight-600 hover:bg-candlelight-400")
                        .dwclass_signal!("bg-candlelight-400", dragging.signal())
                        .event(clone!(dragging => move |_: events::MouseDown| {
                            dragging.set(true);
                        }))
                        .event(|event: events::MouseMove| {
                            event.stop_propagation();
                        })
                        .global_event(clone!(dragging => move |_: events::MouseUp| {
                            dragging.set(false);
                        }))
                    }))
                })
            }))
        }))
    })
}
