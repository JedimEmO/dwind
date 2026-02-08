#[macro_use]
extern crate dominator;

use dominator::{body, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;

#[cfg(not(test))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());

    dominator::replace_dom(&body().parent_node().unwrap(), &body(), main_view());
}

fn main_view() -> Dom {
    dwind::stylesheet();

    html!("div", {
        .dwclass!("font-sans")
        .dwclass!("text-woodsmoke-50 bg-woodsmoke-950")
        .dwclass!("h-screen w-screen flex flex-col justify-center items-center")
        .child(card())
    })
}

fn card() -> Dom {
    html!("div", {
        .dwclass!("bg-woodsmoke-900 rounded-lg p-8 max-w-md")
        .dwclass!("border border-woodsmoke-700 border-solid")
        .dwclass!("shadow-lg")
        .child(html!("h1", {
            .dwclass!("text-2xl font-bold text-picton-blue-400 m-b-4")
            .text("DWIND + Tauri")
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 m-b-6")
            .text("This is a minimal Tauri app using dominator and dwind for the frontend.")
        }))
        .child(html!("div", {
            .dwclass!("flex gap-4")
            .child(html!("button", {
                .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-picton-blue-600 rounded-md")
                .dwclass!("hover:bg-picton-blue-700 transition-colors")
                .dwclass!("text-woodsmoke-50 font-semibold cursor-pointer")
                .text("Primary")
            }))
            .child(html!("button", {
                .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-woodsmoke-700 rounded-md")
                .dwclass!("hover:bg-woodsmoke-600 transition-colors")
                .dwclass!("text-woodsmoke-200 font-semibold cursor-pointer")
                .text("Secondary")
            }))
        }))
    })
}
