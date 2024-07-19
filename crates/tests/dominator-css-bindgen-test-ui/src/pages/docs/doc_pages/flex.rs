use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::{dwclass};
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use dwind_macros::dwgenerate;
use dwind::background_scratched_generator;

pub fn flex_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Flex"))
        .text("Using flex display for adaptable containers")
        .child(flex_example_1())
    })
}
dwgenerate!("background-scratched", "background-scratched-[#2b55a2,#5a6779]");

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
                        .apply_if(v == 0, |b| {
                            dwclass!(b, "flex-none")
                        })
                        .apply(|b| {
                            match v {
                                0 => dwclass!(b, "w-16"),
                                1 => dwclass!(b, "w-64"),
                                _ => dwclass!(b, "w-32"),
                            }
                        })
                        .dwclass!("h-full")
                        .dwclass!("flex align-items-center justify-center")
                        .dwclass!("bg-candlelight-600 rounded-lg font-extrabold")
                        .text(v.to_string().as_str())
                    })
                })
            )
        }))
    })
}