use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::Dom;
use dwind::background_scratched_generator;
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwind_macros::dwgenerate;
use example_html_macro::example_html;

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
    .dwclass!("rounded-lg background-scratched flex gap-2 @md:gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-p-5 @md:w-14 bg-apple-900 flex-none"),
                            1 => dwclass!(b, "w-p-15 @md:w-64 bg-apple-600 flex-initial"),
                            _ => dwclass!(b, "w-p-10 @md:w-32 bg-apple-600 flex-initial"),
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
    .dwclass!("rounded-lg background-scratched flex gap-2 @md:gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-p-5 @md:w-14 bg-charm-900 flex-none"),
                            1 => dwclass!(b, "w-p-15 @md:w-64 bg-charm-600 flex-1"),
                            _ => dwclass!(b, "w-p-10 @md:w-32 bg-charm-600 flex-1"),
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
    .dwclass!("rounded-lg background-scratched flex gap-2 @md:gap-4 align-items-center w-full h-16")
        .children(
            (0..3).map(|v| {
                html!("div", {
                    .apply(|b| {
                        match v {
                            0 => dwclass!(b, "w-5 @md:w-14 bg-candlelight-900 flex-none"),
                            1 => dwclass!(b, "w-15 @md:w-64 bg-candlelight-600 flex-auto"),
                            _ => dwclass!(b, "w-10 @md:w-32 bg-candlelight-600 flex-auto"),
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
