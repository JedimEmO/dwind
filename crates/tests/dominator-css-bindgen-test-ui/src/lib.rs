#[macro_use]
extern crate dominator;

use dominator::Dom;
use dominator_css_bindgen_test::*;
use dwind::base::*;
use dwind_macros::dwclass;
use wasm_bindgen::prelude::wasm_bindgen;
use my_custom_theme::*;
use dwind::color::*;
use dwind::flexbox_and_grid::*;
use dwind::sizing::*;
use dwind::spacing::*;
use dwind::interactivity::*;

#[cfg(not(test))]
#[wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::append_dom(&dominator::body(), main_view());
}

mod my_custom_theme {
    use dwind::interactivity::*;
    use dwind::color::*;
    use dwind_macros::dwgenerate;
    use crate::margin_left_generator;
    use crate::padding_generator;

    dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
    dwgenerate!("hover-margin", "hover:margin-left-[20px]");
    dwgenerate!("hover-bg-apple", "hover:bg-apple-200");
    dwgenerate!("hover-text-apple", "hover:text-apple-950");
}

fn main_view() -> Dom {
    html!("div", {
         .dwclass!("page-body")
         .dwclass!("bg-manatee-950 text-manatee-50")
         .child(header())
         .children((0..1000).map(|_| {
             html!("div", {
                .text("hi there")
                .dwclass!("cursor-pointer hover-bg-apple hover-text-apple bg-bermuda-gray-800")
             })
         }))
    })
}

fn header() -> Dom {
    html!("div", {
        .child(html!("div", {
            .dwclass!("sticky m-x-8 w-p-95 flex justify-stretch align-items-center top-0 height-[60px]")
            .child(html!("div", {
                .child(html!("h3", { .text("dwind") }))
            }))
            .child(html!("div", {
                .dwclass!("m-l-auto m-r-0 flex justify-stretch")
                .children([
                    html!("h3", { .text("examples") }),
                    html!("h3", { .text("docs") }),
                    html!("h3", { .text("github") }),
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