use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn borders_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Borders & Rounding"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Border sides, styles, colors, corner radii, and sibling dividers.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["border".to_string(), "1px border on all sides".to_string()],
                ["border-{l|r|t|b}".to_string(), "Border on one side".to_string()],
                ["border-{solid|dashed|dotted|double|none}".to_string(), "Border style".to_string()],
                ["border-{color}-{shade}".to_string(), "Border color from any palette".to_string()],
                ["rounded-{sm|md|lg|xl|2xl|3xl|full|none}".to_string(), "Corner radius (rounded = default)".to_string()],
                ["rounded-{t|b|l|r|tl|tr|bl|br}-{...}".to_string(), "Per-side / per-corner radius".to_string()],
                ["divide-{x|y}".to_string(), "1px dividers between siblings".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(borders_example(), false))
        .child(code(&BORDERS_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn borders_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-4 w-full justify-center font-mono text-xs text-woodsmoke-300")
        .child(html!("div", {
            .dwclass!("w-28 h-16 border border-candlelight-500 rounded-md flex align-items-center justify-center")
            .text("solid")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 border border-dashed border-picton-blue-400 rounded-lg flex align-items-center justify-center")
            .text("dashed")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 border border-dotted border-charm-400 rounded-xl flex align-items-center justify-center")
            .text("dotted")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 border border-apple-400 rounded-full flex align-items-center justify-center")
            .text("rounded-full")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 border-l border-candlelight-500 flex align-items-center justify-center")
            .text("border-l")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-t-xl bg-woodsmoke-800 flex align-items-center justify-center")
            .text("rounded-t-xl")
        }))
    })
}
