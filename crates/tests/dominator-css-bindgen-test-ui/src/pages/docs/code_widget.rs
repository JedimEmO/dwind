use std::collections::BTreeMap;
use dominator::Dom;
use dwind_macros::dwclass;
use dwind::prelude::*;
use crate::pages::docs::example_box::example_box;

pub fn code(example_map: &BTreeMap<String, String>) -> Dom {
    html!("div", {
        .dwclass!("rounded-lg bg-bunker-950 w-full m-t-10")
        .dwclass!("border border-woodsmoke-800")
        .child(html!("code", {
            .dwclass!("h-full")
            .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
        }))
    })
}