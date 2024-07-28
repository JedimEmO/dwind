mod pages;
mod router;

#[macro_use]
extern crate log;

#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwui;

use crate::pages::docs::doc_main::doc_main_view;
use crate::pages::docs::doc_sidebar::doc_sidebar;
use crate::pages::docs::{doc_sections, DocPage};
use crate::router::AppRouter;
use dominator::Dom;
use dwind::base::*;
use dwind::borders::*;
use dwind::colors::*;
use dwind::flexbox_and_grid::*;
use dwind::sizing::*;
use dwind::spacing::*;
use dwind::typography::*;
use dwind_macros::dwclass;
use futures_signals::signal::SignalExt;
use matchit::Params;
use std::sync::Arc;
use wasm_bindgen::UnwrapThrowExt;

#[cfg(not(test))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::append_dom(&dominator::body(), main_view());
}

mod my_custom_theme {
    use crate::margin_left_generator;
    use crate::padding_generator;
    use dwind::prelude::*;
    use dwind_macros::dwgenerate;

    dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
    dwgenerate!("hover-margin", "hover:margin-left-[20px]");
    dwgenerate!("hover-bg-apple", "hover:bg-apple-200");
    dwgenerate!("hover-text-apple", "hover:text-apple-950");
}

fn make_app_router() -> AppRouter<DocPage> {
    let mut router = matchit::Router::<Box<dyn Fn(Params) -> Result<DocPage, ()>>>::new();

    router
        .insert("#/docs/colors", Box::new(|_| Ok(DocPage::Colors)))
        .unwrap_throw();

    router
        .insert("#/docs/flex", Box::new(|_| Ok(DocPage::Flex)))
        .unwrap_throw();

    router
        .insert(
            "#/docs/responsive-design",
            Box::new(|_| Ok(DocPage::ResponsiveDesign)),
        )
        .unwrap_throw();

    AppRouter::new(router)
}
fn main_view() -> Dom {
    let router = make_app_router();

    dwind::stylesheet();

    stylesheet!(["body"], {
        // Use the generated DWIND_COLORS map if we need to programatically access color values
        .style("background-color", &DWIND_COLORS["woodsmoke"][&950])
        .style("overflow-y", "scroll")
    });

    html!("div", {
        .dwclass!("font-sans")
        .dwclass!("text-woodsmoke-200")
        .child(header())
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-lg flex h-p-90")
            .style("margin-top", "4px")
            .child(doc_sidebar(doc_sections(), make_app_router().signal(), Arc::new(|v: DocPage| v.goto())))
            .child(html!("div", {
                .dwclass!("m-l-4 m-r-0 w-p-75 @md:w-full")
                .child(doc_main_view(router.signal().map(Some)))
            }))
        }))
    })
}

fn header() -> Dom {
    html!("div", {
        .child(html!("div", {
            .dwclass!("border-b border-woodsmoke-800 border-solid")
            .dwclass!("font-extrabold")
            .dwclass!("sticky m-x-auto max-w-lg flex justify-stretch align-items-center top-0 h-12")
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
