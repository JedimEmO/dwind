use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use std::collections::BTreeMap;

pub fn code(example_map: &BTreeMap<String, String>) -> Dom {
    let example = html!("code", {
        .dwclass!("h-full")
        .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
    });

    card!({
        .apply(move |b| {
            let b = dwclass!(b, "rounded-lg m-t-10 overflow-x-scroll @md:overflow-x-auto");
            let b = dwclass!(b, "border border-woodsmoke-800");

            b.child(example)
        })
    })
}
