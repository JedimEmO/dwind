use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn filters_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Filters & Effects"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Blur, brightness, grayscale, backdrop filters, and opacity.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["blur-{sm|md|lg|xl|2xl|3xl|none}".to_string(), "Gaussian blur (blur = default)".to_string()],
                ["brightness-{0|50|75|90|100|110|125|150|200}".to_string(), "Brightness — pairs well with hover:".to_string()],
                ["grayscale / grayscale-0".to_string(), "Desaturation".to_string()],
                ["backdrop-blur-{sm..3xl}".to_string(), "Blur what's behind the element".to_string()],
                ["opacity-{0|5|10|…|95|100}".to_string(), "Element opacity".to_string()],
                ["filter-none".to_string(), "Remove all filters".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(filters_example(), false))
        .child(code(&FILTERS_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn filters_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-6 w-full justify-center font-mono text-xs text-woodsmoke-950")
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg linear-gradient-45 gradient-from-candlelight-300 gradient-to-charm-500")
            .dwclass!("flex align-items-center justify-center")
            .text("original")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg linear-gradient-45 gradient-from-candlelight-300 gradient-to-charm-500")
            .dwclass!("flex align-items-center justify-center blur-sm")
            .text("blur-sm")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg linear-gradient-45 gradient-from-candlelight-300 gradient-to-charm-500")
            .dwclass!("flex align-items-center justify-center grayscale")
            .text("grayscale")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg linear-gradient-45 gradient-from-candlelight-300 gradient-to-charm-500")
            .dwclass!("flex align-items-center justify-center brightness-50")
            .text("brightness-50")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg linear-gradient-45 gradient-from-candlelight-300 gradient-to-charm-500")
            .dwclass!("flex align-items-center justify-center opacity-30")
            .text("opacity-30")
        }))
        .child(html!("div", {
            .dwclass!("w-28 h-16 rounded-lg bg-woodsmoke-800 hover:brightness-150 transition-all cursor-pointer")
            .dwclass!("flex align-items-center justify-center text-woodsmoke-200")
            .text("hover:brightness-150")
        }))
    })
}
