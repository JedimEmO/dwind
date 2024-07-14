#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwind;

use dominator::Dom;
use wasm_bindgen::prelude::wasm_bindgen;
use dominator_css_bindgen_test::*;
use dwind::base::*;
use dwind_macros::{dwclass, dwgenerate};

macro_rules! padding_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding: {};", $padding)
    };
}

macro_rules! margin_left_generator {
    ($margin_left:tt) => {
        const_format::formatcp!("margin-left: {};", $margin_left)
    };
}

#[wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::append_dom(
        &dominator::body(),
        main_view(),
    );
}

mod my_custom_theme {
    use dwind_macros::dwgenerate;

    dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
    dwgenerate!("hover-margin", "hover:margin-left-[20px]");
}

fn main_view() -> Dom {
    use my_custom_theme::*;

    html!("div", {
        .dwclass!("page-body")
        .dwclass!("bg-slate-900 text-slate-500")
        .child(html!("div", {
            .dwclass!("sticky top-0 height-[60px]")
        }))
        .children((0..1000).map(|_| {
            html!("div", {
               .text("hi there")
               .dwclass!("nth-2-padding hover-margin")
            })
        }))
   })
}

