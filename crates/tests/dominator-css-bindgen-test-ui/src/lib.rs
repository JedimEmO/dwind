#[macro_use]
extern crate dominator;

use dominator::Dom;
use dominator_css_bindgen_test::*;
use dwind::base::*;
use dwind_macros::dwclass;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(test))]
#[wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::append_dom(&dominator::body(), main_view());
}

mod my_custom_theme {
    use dwind_macros::dwgenerate;
    use crate::margin_left_generator;
    use crate::padding_generator;

    dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
    dwgenerate!("hover-margin", "hover:margin-left-[20px]");
}

fn main_view() -> Dom {
    use my_custom_theme::*;
    use dwind::color::*;
    use dwind::interactivity::*;

    html!("div", {
         .dwclass!("page-body")
         .dwclass!("bg-bermuda-gray-950 text-bermuda-gray-50")
         .child(html!("div", {
             .dwclass!("sticky top-0 height-[60px]")
         }))
         .children((0..1000).map(|_| {
             html!("div", {
                .text("hi there")
                .dwclass!("cursor-pointer bg-bermuda-gray-300")
             })
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