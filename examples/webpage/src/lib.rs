mod pages;
mod router;

#[macro_use]
extern crate log;

#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwui;

use crate::pages::docs::doc_main::doc_main_view;
use crate::pages::docs::doc_sidebar::doc_sidebar;
use crate::pages::docs::{doc_sections, DocPage};
use crate::router::make_app_router;
use dominator::routing::go_to_url;
use dominator::{body, events, Dom};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use dwui::theme::prelude::ColorsCssVariables;
use futures_signals::signal::{always, Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use gloo_timers::future::sleep;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

fn child(count: impl Signal<Item = u32>) -> impl Signal<Item = Dom> {
    count.map(|count| html!("div", { .text(&format!("{count}")) }))
}

fn container(data: impl SignalVec<Item = u32> + 'static) -> Dom {
    let counter = Mutable::new(0);

    let children = data.map_signal(clone!(counter => move |v| {
        child(counter.signal())
    }));

    html!("div", {
        .children_signal_vec(children)
    })
}

#[cfg(not(test))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::replace_dom(&body().parent_node().unwrap(), &body(), main_view());
    // dominator::replace_dom(&body().parent_node().unwrap(), &body(), memleaktest());
}

fn memleaktest() -> Dom {
    let data = MutableVec::new_with_values((0..100).collect());

    // Create a cloned reference to pass into the async task
    let data_clone = data.clone();

    // Generate new values (you can modify this pattern)

    // Spawn an async task that will update the vector periodically
    spawn_local(async move {
        let mut count = 0;

        loop {
            let new_values: Vec<i32> = (count..count + 100).collect();

            // Replace the content of the mutable vec
            data_clone.lock_mut().replace(new_values);

            // Sleep for 100ms
            sleep(Duration::from_millis(100)).await;

            count += 1;
        }
    });

    html!("div", {
        .children_signal_vec(data.signal_vec().map(|v| {
           html!("div", {
               .dwclass!("hover:text-red-500 @sm:text-red-500 is(.light *):bg-red-500")
               // .dwclass_signal!("hover:text-red-500 @sm:text-red-500 is(.light *):bg-red-500", always(true))
               .text(&format!("{v}"))
            })
        }))
    })
}
fn main_view() -> Dom {
    dwind::stylesheet();
    dwui::theme::apply_style_sheet(Some(ColorsCssVariables::new(
        &DWIND_COLORS["apple"],
        &DWIND_COLORS["woodsmoke"],
        &DWIND_COLORS["woodsmoke"],
        &DWIND_COLORS["red"],
    )));

    html!("div", {
        .dwclass!("font-sans")
        .dwclass!("text-woodsmoke-50 bg-woodsmoke-950")
        .dwclass!("h-full overflow-y-scroll")
        .child(header())
        .child(html!("div", {
            .dwclass!("m-x-auto flex max-w-lg")
            .style("margin-top", "4px")
            .child_signal(doc_sidebar(doc_sections(), || make_app_router().signal(), Arc::new(|v: DocPage| v.goto()), || {
                html!("div", {
                    .dwclass!("m-l-4 m-r-0")
                    .child_signal(doc_main_view(make_app_router().signal().map(Some)))
                })
            }))
        }))
    })
}

fn header() -> Dom {
    html!("div", {
        .child(html!("div", {
            .dwclass!("border-b border-woodsmoke-800 border-solid")
            .dwclass!("font-extrabold")
            .dwclass!("sticky m-x-auto max-w-lg flex justify-stretch align-items-center top-0 h-12")
            .child(html!("div", {
                .child(html!("h3", { .text("dwind") }))
            }))
            .child(html!("div", {
                .dwclass!("m-l-auto m-r-0 flex justify-stretch")
                .children([
                    html!("h3", {
                        .text("examples")
                        .dwclass!("m-x-2 text-picton-blue-200 hover:text-picton-blue-500 hover:font-bold cursor-pointer")
                        .event(|_: events::Click| {
                            go_to_url("#/examples")
                        })
                    }),
                    html!("h3", {
                        .text("dwui")
                        .dwclass!("m-x-2 text-picton-blue-200 hover:text-picton-blue-500 hover:font-bold cursor-pointer")
                        .event(|_: events::Click| {
                            go_to_url("#/dwui-examples")
                        })
                    }),
                    html!("h3", {
                        .text("docs")
                        .dwclass!("m-x-2 hover:text-picton-blue-200 hover:font-bold cursor-pointer")
                        .event(|_: events::Click| {
                            window().unwrap().open_with_url_and_target("https://jedimemo.github.io/dwind/doc/dwind/index.html", "_blank").unwrap();
                        })
                    }),
                    html!("h3", {
                        .text("github")
                        .dwclass!("m-x-2 hover:text-picton-blue-200 hover:font-bold cursor-pointer")
                        .event(|_: events::Click| {
                            window().unwrap().open_with_url_and_target("https://github.com/JedimEmO/dwind", "_blank").unwrap();
                        })
                    }),
                ])
            }))
         }))
    })
}
