use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn spacing_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Spacing"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Padding, margin, and gap utilities. The numeric scale is rem-based: \
                   one unit is 0.25rem, so p-4 is 1rem of padding.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["p-{n}".to_string(), "Padding on all sides".to_string()],
                ["p-{l|r|t|b}-{n}".to_string(), "Padding on one side (also pl-/pr-/pt-/pb-)".to_string()],
                ["px-{n} / py-{n}".to_string(), "Horizontal / vertical padding".to_string()],
                ["m-{n}, m-{l|r|t|b}-{n}".to_string(), "Margin, same patterns as padding".to_string()],
                ["m-x-auto / mx-auto".to_string(), "Horizontal centering".to_string()],
                ["gap-{n}".to_string(), "Gap between flex/grid children".to_string()],
                ["space-x-{n} / space-y-{n}".to_string(), "Margin between siblings".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(spacing_example(), false))
        .child(code(&SPACING_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn spacing_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4 w-full")
        .child(html!("div", {
            .dwclass!("bg-woodsmoke-800 rounded-lg p-2 w-fit")
            .child(html!("div", {
                .dwclass!("bg-candlelight-500 rounded-md p-2 text-woodsmoke-950 font-mono text-sm")
                .text("p-2")
            }))
        }))
        .child(html!("div", {
            .dwclass!("bg-woodsmoke-800 rounded-lg p-6 w-fit")
            .child(html!("div", {
                .dwclass!("bg-candlelight-500 rounded-md p-2 text-woodsmoke-950 font-mono text-sm")
                .text("p-6")
            }))
        }))
        .child(html!("div", {
            .dwclass!("flex flex-row gap-4")
            .children((0..3).map(|_| {
                html!("div", {
                    .dwclass!("bg-picton-blue-800 rounded-md w-16 h-10 flex align-items-center justify-center font-mono text-xs")
                    .text("gap-4")
                })
            }))
        }))
        .child(html!("div", {
            .dwclass!("m-x-auto bg-charm-800 rounded-md p-l-4 p-r-4 p-t-1 p-b-1 font-mono text-xs w-fit")
            .text("m-x-auto centers me")
        }))
    })
}
