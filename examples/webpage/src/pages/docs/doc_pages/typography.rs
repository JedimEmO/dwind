use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn typography_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Typography"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Font sizes, weights, line heights, families, and text alignment.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["text-{xs|sm|base|l|lg|xl|2xl|…|9xl}".to_string(), "Font size scale".to_string()],
                ["font-{thin|light|normal|medium|semibold|bold|extrabold|black}".to_string(), "Font weight".to_string()],
                ["font-{sans|serif|mono}".to_string(), "Font family".to_string()],
                ["leading-{none|tight|snug|normal|relaxed|loose|3..10}".to_string(), "Line height".to_string()],
                ["text-{left|center|right}".to_string(), "Text alignment".to_string()],
                ["truncate / text-ellipsis / text-clip".to_string(), "Overflow handling".to_string()],
                ["text-{color}-{shade}".to_string(), "Text color from any palette".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Specimen"))
        .child(example_box(typography_example(), false))
        .child(code(&TYPOGRAPHY_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn typography_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-3 w-full")
        .child(html!("div", {
            .dwclass!("text-3xl font-extrabold text-woodsmoke-50")
            .text("text-3xl font-extrabold")
        }))
        .child(html!("div", {
            .dwclass!("text-l font-semibold text-woodsmoke-100")
            .text("text-l font-semibold")
        }))
        .child(html!("div", {
            .dwclass!("text-base leading-relaxed text-woodsmoke-300")
            .text("text-base leading-relaxed — comfortable body copy with breathing room between lines, ideal for prose.")
        }))
        .child(html!("div", {
            .dwclass!("text-sm font-mono text-candlelight-300")
            .text("text-sm font-mono")
        }))
        .child(html!("div", {
            .dwclass!("text-center text-woodsmoke-400 text-sm")
            .text("text-center")
        }))
        .child(html!("div", {
            .dwclass!("truncate w-64 text-woodsmoke-300 text-sm")
            .text("truncate — this very long line of text will be cut off with an ellipsis rather than wrapping")
        }))
    })
}
