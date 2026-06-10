use dominator::{events, Dom};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use futures_signals::signal::not;
use futures_signals::signal::{Mutable, SignalExt};
use std::collections::BTreeMap;

/// A collapsible, syntax-highlighted source listing for an example.
pub fn code(example_map: &BTreeMap<String, String>) -> Dom {
    let expanded = Mutable::new(false);
    let example_map = example_map.clone();

    html!("div", {
        .dwclass!("rounded-lg m-t-2 border border-woodsmoke-800 overflow-hidden w-full")
        .child(html!("button", {
            .attr("type", "button")
            .attr_signal("aria-expanded", expanded.signal().map(|v| if v { "true" } else { "false" }))
            .class("font-code")
            .dwclass!("flex flex-row align-items-center gap-2 w-full p-l-4 p-r-4 h-10 cursor-pointer text-left text-sm")
            .dwclass!("bg-transparent border-none text-woodsmoke-400 hover:text-candlelight-300 transition-colors")
            .style("outline", "none")
            .child(html!("span", {
                .dwclass!("inline-block transition-transform")
                .style_signal("transform", expanded.signal().map(|v| {
                    if v { "rotate(90deg)" } else { "rotate(0deg)" }
                }))
                .text("▸")
            }))
            .child(html!("span", { .text("view source") }))
            .event(clone!(expanded => move |_: events::Click| {
                expanded.set(!expanded.get());
            }))
        }))
        .child(html!("div",{
            .dwclass!("overflow-x-auto transition-all")
            .dwclass_signal!("max-h-0 overflow-y-hidden", not(expanded.signal()))
            .dwclass_signal!("max-h-md overflow-y-auto border-t border-woodsmoke-800", expanded.signal())
            .style("background", "rgba(2, 2, 3, 0.7)")
            .child(html!("code", {
                .class("font-code")
                .dwclass!("text-sm block p-4")
                .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
            }))
        }))
    })
}
