use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use dwui::prelude::*;
use std::collections::BTreeMap;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal::not;

pub fn code(example_map: &BTreeMap<String, String>) -> Dom {
    let expanded = Mutable::new(false);
    let example_map = example_map.clone();

    card!({
        .apply(move |b| {
            let b = dwclass!(b, "rounded-lg m-t-10 @md:overflow-x-auto");
            let b = dwclass!(b, "border border-woodsmoke-800");

            b.child(html!("div", {
                .dwclass!("font-extrabold cursor-pointer p-4 hover:text-picton-blue-500")
                .text("Show example code")
                .event(clone!(expanded => move |_: events::Click| {
                    expanded.set(!expanded.get());
                }))
            }))
            .child(html!("div",{
                .dwclass!("overflow-x-auto overflow-y-scroll max-h-md transition-all")
                .dwclass_signal!("max-h-0 overflow-y-hidden", not(expanded.signal()))
                .dwclass_signal!("max-h-md overflow-y-scroll", expanded.signal())
                .child(html!("code", {
                    .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
                }))
            }))
        })
    })
}
