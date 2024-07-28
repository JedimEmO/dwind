use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use std::collections::BTreeMap;

pub fn code(example_map: &BTreeMap<String, String>) -> Dom {
    html!("div", {
        .dwclass!("rounded-lg bg-bunker-950 w-full m-t-10 overflow-x-scroll @md:overflow-x-auto")
        .dwclass!("border border-woodsmoke-800")
        .child(html!("code", {
            .dwclass!("h-full")
            .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
        }))
    })
}
