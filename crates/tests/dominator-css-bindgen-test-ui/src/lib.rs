#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwui;

use dominator::{Dom, text};
use dominator_css_bindgen_test::*;
use dwind::base::*;
use dwind::borders::*;
use dwind::colors::*;
use dwind::flexbox_and_grid::*;
use dwind::interactivity::*;
use dwind::sizing::*;
use dwind::spacing::*;
use dwind::typography::*;
use dwind_macros::dwclass;
use my_custom_theme::*;
use wasm_bindgen::prelude::wasm_bindgen;
use dwui::prelude::*;

#[cfg(not(test))]
#[wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::append_dom(&dominator::body(), main_view());
}

mod my_custom_theme {
    use crate::margin_left_generator;
    use crate::padding_generator;
    use dwind::colors::*;
    use dwind::interactivity::*;
    use dwind_macros::dwgenerate;

    dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
    dwgenerate!("hover-margin", "hover:margin-left-[20px]");
    dwgenerate!("hover-bg-apple", "hover:bg-apple-200");
    dwgenerate!("hover-text-apple", "hover:text-apple-950");
}

fn main_view() -> Dom {
    html!("div", {
        .dwclass!("page-body font-sans")
        .dwclass!("bg-manatee-950 text-manatee-50")
        .child(header())
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-lg flex")
            .style("margin-top", "4px")
            .child(html!("div", {
                .dwclass!("w-32 m-l-0 border-r border-color-manatee-800 border-solid")
                .text("hi")
            }))
            .child(html!("div", {
                .dwclass!("m-l-4 m-r-0 w-full")
                .children((0..20).map(|_| {
                    card!({
                        .content(heading!({
                            .content(text("Hi there!"))
                        }))
                    })
                }))
            }))
        }))
    })
}

fn header() -> Dom {
    html!("div", {
        .child(html!("div", {
            .dwclass!("border-b border-color-manatee-800 border-solid")
            .dwclass!("sticky m-x-auto max-w-lg flex justify-stretch align-items-center top-0 height-[60px]")
            .child(html!("div", {
                .child(html!("h3", { .text("dwind") }))
            }))
            .child(html!("div", {
                .dwclass!("m-l-auto m-r-0 flex justify-stretch")
                .children([
                    html!("h3", { .text("examples").dwclass!("m-x-2") }),
                    html!("h3", { .text("docs").dwclass!("m-x-2") }),
                    html!("h3", { .text("github").dwclass!("m-x-2") }),
                ])
            }))
         }))
    })
}
mod generators {
    #[macro_export]
    macro_rules! padding_generator {
        ($padding:tt) => {
            const_format::formatcp!("padding: {};", $padding)
        };
    }

    #[macro_export]
    macro_rules! margin_left_generator {
        ($margin_left:tt) => {
            const_format::formatcp!("margin-left: {};", $margin_left)
        };
    }

    #[macro_export]
    macro_rules! height_generator {
        ($height:tt) => {
            const_format::formatcp!("height: {};", $height)
        };
    }
}
