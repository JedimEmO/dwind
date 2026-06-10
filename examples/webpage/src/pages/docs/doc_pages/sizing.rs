use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn sizing_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Sizing"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Width and height utilities: the rem scale, percentages, viewport units, \
                   and content-based sizing.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["w-{n} / h-{n}".to_string(), "Fixed size on the 0.25rem scale (w-10 = 2.5rem)".to_string()],
                ["w-full / h-full".to_string(), "100% of the parent".to_string()],
                ["w-screen / h-screen".to_string(), "100vw / 100vh".to_string()],
                ["w-p-{5..95} / h-p-{5..95}".to_string(), "Percentage sizes in steps of 5".to_string()],
                ["w-{fit|min|max}".to_string(), "fit-content / min-content / max-content".to_string()],
                ["max-w-{sm..7xl} / max-w-screen-{sm..2xl}".to_string(), "Maximum widths".to_string()],
                ["min-w-{...} / min-h-{...} / max-h-{...}".to_string(), "Min/max constraints, same scales".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(sizing_example(), false))
        .child(code(&SIZING_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn sizing_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-2 w-full")
        .children([
            ("w-16", "w-16"),
            ("w-32", "w-32"),
            ("w-64", "w-64"),
            ("w-p-50", "w-p-50"),
            ("w-p-75", "w-p-75"),
            ("w-full", "w-full"),
        ].map(|(label, _)| {
            html!("div", {
                .apply(|b| match label {
                    "w-16" => dwclass!(b, "w-16"),
                    "w-32" => dwclass!(b, "w-32"),
                    "w-64" => dwclass!(b, "w-64"),
                    "w-p-50" => dwclass!(b, "w-p-50"),
                    "w-p-75" => dwclass!(b, "w-p-75"),
                    _ => dwclass!(b, "w-full"),
                })
                .dwclass!("bg-candlelight-600 rounded-md h-8 p-l-3 flex align-items-center")
                .dwclass!("font-mono text-xs text-woodsmoke-950 overflow-hidden")
                .text(label)
            })
        }))
    })
}
