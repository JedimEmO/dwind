use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn transforms_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Transforms & Transitions"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Scale, rotate, translate, and skew — and the transition utilities that animate them.")
        }))
        .child(doc_page_sub_header("Transforms"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["scale-{0|50|75|90|95|100|105|110|125|150}".to_string(), "Uniform scale (also scale-x-/scale-y-)".to_string()],
                ["rotate-{0|1|2|3|6|12|45|90|180}".to_string(), "Rotation in degrees".to_string()],
                ["translate-x-{n} / translate-y-{n}".to_string(), "Translation on the spacing scale".to_string()],
                ["skew-x-{n} / skew-y-{n}".to_string(), "Skew in degrees".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Transitions"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["transition".to_string(), "Sensible default property set".to_string()],
                ["transition-{all|colors|opacity|shadow|transform|none}".to_string(), "Property groups".to_string()],
                ["duration-{75|100|150|200|300|500|700|1000}".to_string(), "Duration in ms".to_string()],
                ["ease-{linear|in|out|in-out}".to_string(), "Timing function".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(transforms_example(), false))
        .child(code(&TRANSFORMS_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn transforms_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-6 w-full justify-center p-4 font-mono text-xs text-woodsmoke-200")
        .child(html!("div", {
            .dwclass!("w-32 h-16 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
            .dwclass!("transition-transform duration-200 ease-out hover:scale-110 cursor-pointer")
            .dwclass!("flex align-items-center justify-center")
            .text("hover:scale-110")
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-16 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
            .dwclass!("transition-transform duration-300 ease-in-out hover:rotate-6 cursor-pointer")
            .dwclass!("flex align-items-center justify-center")
            .text("hover:rotate-6")
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-16 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
            .dwclass!("transition-transform duration-500 hover:translate-y-2 cursor-pointer")
            .dwclass!("flex align-items-center justify-center")
            .text("hover:translate-y-2")
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-16 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
            .dwclass!("transition-transform duration-200 hover:skew-x-6 cursor-pointer")
            .dwclass!("flex align-items-center justify-center")
            .text("hover:skew-x-6")
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-16 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
            .dwclass!("transition-all duration-700 hover:rotate-3 hover:scale-105 hover:bg-candlelight-500 hover:text-woodsmoke-950 cursor-pointer")
            .dwclass!("flex align-items-center justify-center")
            .text("composed")
        }))
    })
}
