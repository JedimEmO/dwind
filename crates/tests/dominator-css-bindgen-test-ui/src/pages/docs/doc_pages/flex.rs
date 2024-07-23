use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use dominator::{Dom, text};
use dwind::background_scratched_generator;
use dwind::prelude::*;
use dwui::prelude::*;
use crate::dwui::heading;
use dwind_macros::dwclass;
use dwind_macros::dwgenerate;
use example_html_macro::example_html;
use std::collections::BTreeMap;
use dwui::prelude::TextSize;
use crate::pages::docs::code_widget::code;
use crate::pages::docs::example_box::example_box;

pub fn flex_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Flex"))
        .text("Using flex display for adaptable containers")
        .child(doc_page_sub_header("flex-initial"))
        .child(html!("p", {
            .text("Allows elements to shrink but not expand to fit the flex box")
        }))
        .child(example_box(flex_example_initial(), true))
        .child(code(&FLEX_EXAMPLE_INITIAL_EXAMPLE_HTML_MAP))
        .child(doc_page_sub_header("flex-1"))
        .child(html!("p", {
            .text("Scales elements to fit the flex box")
        }))
        .child(example_box(flex_example_1(), true))
        .child(code(&FLEX_EXAMPLE_1_EXAMPLE_HTML_MAP))
        .child(doc_page_sub_header("flex-auto"))
        .child(html!("p", {
            .text("Scales elements to fit the flex box, relative to their original size")
        }))
        .child(example_box(flex_example_auto(), true))
        .child(code(&FLEX_EXAMPLE_AUTO_EXAMPLE_HTML_MAP))
    })
}
dwgenerate!(
    "background-scratched",
    "background-scratched-[#1c2e4f,#191919]"
);

#[rustfmt::skip]
#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn flex_example_initial() -> Dom {
html!("div", {
    .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                            1 => dwclass!(b, "w-64 bg-candlelight-500 flex-initial"),
                            _ => dwclass!(b, "w-32 bg-candlelight-500 flex-initial"),
                        }
                    })
                    .dwclass!("h-full")
                    .dwclass!("flex align-items-center justify-center")
                    .dwclass!("rounded-lg font-extrabold")
                    .text(v.to_string().as_str())
                })
            })
        )
})
}


#[rustfmt::skip]
#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn flex_example_1() -> Dom {
    html!("div", {
    .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                            1 => dwclass!(b, "w-64 bg-candlelight-500 flex-1"),
                            _ => dwclass!(b, "w-32 bg-candlelight-500 flex-1"),
                        }
                    })
                    .dwclass!("h-full")
                    .dwclass!("flex align-items-center justify-center")
                    .dwclass!("rounded-lg font-extrabold")
                    .text(v.to_string().as_str())
                })
            })
        )
})
}


#[rustfmt::skip]
#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn flex_example_auto() -> Dom {
    html!("div", {
    .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                            1 => dwclass!(b, "w-64 bg-candlelight-500 flex-auto"),
                            _ => dwclass!(b, "w-32 bg-candlelight-500 flex-auto"),
                        }
                    })
                    .dwclass!("h-full")
                    .dwclass!("flex align-items-center justify-center")
                    .dwclass!("rounded-lg font-extrabold")
                    .text(v.to_string().as_str())
                })
            })
        )
})
}

