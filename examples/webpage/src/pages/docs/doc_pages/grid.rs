use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn grid_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Grid"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("CSS grid utilities: column templates, spans, placement, and flow.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["grid / inline-grid".to_string(), "Grid containers".to_string()],
                ["grid-cols-{1..12|none}".to_string(), "Column template".to_string()],
                ["col-span-{1..12|full}".to_string(), "Column span".to_string()],
                ["col-start-{n} / col-end-{n}".to_string(), "Explicit placement".to_string()],
                ["grid-row-{n} / row-span-{n}".to_string(), "Row placement and span".to_string()],
                ["grid-flow-{row|col|dense}".to_string(), "Auto-placement flow".to_string()],
                ["auto-cols-{auto|fr|min|max}".to_string(), "Implicit track sizing".to_string()],
                ["gap-{n}".to_string(), "Track gap (shared with flex)".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(grid_example(), true))
        .child(code(&GRID_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn grid_example() -> Dom {
    html!("div", {
        .dwclass!("grid grid-cols-6 gap-2 w-full font-mono text-xs text-woodsmoke-950")
        .child(html!("div", {
            .dwclass!("col-span-6 bg-candlelight-400 rounded-md h-10 flex align-items-center justify-center")
            .text("col-span-6")
        }))
        .child(html!("div", {
            .dwclass!("col-span-4 bg-picton-blue-400 rounded-md h-10 flex align-items-center justify-center")
            .text("col-span-4")
        }))
        .child(html!("div", {
            .dwclass!("col-span-2 bg-charm-400 rounded-md h-10 flex align-items-center justify-center")
            .text("col-span-2")
        }))
        .children((0..3).map(|_| {
            html!("div", {
                .dwclass!("col-span-2 bg-apple-400 rounded-md h-10 flex align-items-center justify-center")
                .text("col-span-2")
            })
        }))
        .child(html!("div", {
            .dwclass!("col-start-2 col-span-4 bg-woodsmoke-300 rounded-md h-10 flex align-items-center justify-center")
            .text("col-start-2 col-span-4")
        }))
    })
}
