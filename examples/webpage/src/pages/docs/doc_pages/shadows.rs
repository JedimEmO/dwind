use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn shadows_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Shadows & Rings"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Box shadows for elevation, and rings — outline-like box shadows that \
                   don't affect layout, ideal for focus states.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["shadow-{sm|md|lg|xl|2xl}".to_string(), "Elevation scale (shadow = default)".to_string()],
                ["shadow-inner".to_string(), "Inset shadow".to_string()],
                ["shadow-none".to_string(), "Remove shadow".to_string()],
                ["ring-{0|1|2|4|8}".to_string(), "Ring width (ring = 3px)".to_string()],
                ["ring-{color}-{shade}".to_string(), "Ring color from any palette".to_string()],
                ["ring-inset".to_string(), "Draw the ring inside the element".to_string()],
                ["focus-visible:ring-2".to_string(), "Typical keyboard focus indicator".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(shadows_example(), false))
        .child(code(&SHADOWS_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn shadows_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-6 w-full justify-center p-4 font-mono text-xs")
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-200 shadow-sm flex align-items-center justify-center text-woodsmoke-900")
            .text("shadow-sm")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-200 shadow-lg flex align-items-center justify-center text-woodsmoke-900")
            .text("shadow-lg")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-200 shadow-2xl flex align-items-center justify-center text-woodsmoke-900")
            .text("shadow-2xl")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-900 ring-2 ring-candlelight-400 flex align-items-center justify-center text-woodsmoke-200")
            .text("ring-2")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-900 ring-4 ring-charm-500 ring-inset flex align-items-center justify-center text-woodsmoke-200")
            .text("ring-inset")
        }))
        .child(html!("button", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-800 border-none cursor-pointer text-woodsmoke-200")
            .dwclass!("focus-visible:ring-2 ring-picton-blue-400")
            .style("outline", "none")
            .text("tab to me")
        }))
    })
}
