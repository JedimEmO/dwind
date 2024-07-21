use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use dominator::Dom;
use dwind::background_scratched_generator;
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwind_macros::dwgenerate;
use example_html_macro::example_html;
use std::collections::BTreeMap;

pub fn flex_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Flex"))
        .text("Using flex display for adaptable containers")
        .child(flex_example_1())
        .child(code(&FLEX_EXAMPLE_1_EXAMPLE_HTML_MAP))
    })
}
dwgenerate!(
    "background-scratched",
    "background-scratched-[#2b55a2,#5a6779]"
);

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn flex_example_1() -> Dom {
    html!("div", {
        .dwclass!("rounded-lg bg-bunker-950 w-full h-32")
        .dwclass!("border border-color-manatee-800")
        .dwclass!("flex align-items-center p-5 m-t-10")
        .child(html!("div", {
            .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-p-60")
            .children(
                (0..3).map(|v| {
                    html!("div", {
                        .apply(|b| {
                            match v {
                                0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                                1 => dwclass!(b, "w-64 bg-candlelight-600 flex-initial"),
                                _ => dwclass!(b, "w-32 bg-candlelight-600 flex-initial"),
                            }
                        })
                        .dwclass!("h-full")
                        .dwclass!("flex align-items-center justify-center")
                        .dwclass!("rounded-lg font-extrabold")
                        .text(v.to_string().as_str())
                    })
                })
            )
        }))
    })
}

fn code(example_map: &BTreeMap<String, String>) -> Dom {
    html!("pre", {
        .child(html!("code", {
            .prop("innerHTML", example_map["base16-ocean.dark"].as_str())
        }))
        .child(html!("code", {
            .prop("innerHTML", example_map["base16-ocean.light"].as_str())
        }))
    })
}
