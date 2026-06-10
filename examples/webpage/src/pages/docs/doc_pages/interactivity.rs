use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn interactivity_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Interactivity"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Cursors, text selection, and pointer-event control.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["cursor-{pointer|default|text|move|grab|grabbing|…}".to_string(), "Cursor shape (30+ variants)".to_string()],
                ["cursor-{wait|progress|not-allowed|help}".to_string(), "Status cursors".to_string()],
                ["select-{none|text|all|auto}".to_string(), "Text selection behavior".to_string()],
                ["pointer-events-{none|auto}".to_string(), "Hit-testing control".to_string()],
                ["appearance-{none|auto}".to_string(), "Native form control styling".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(interactivity_example(), false))
        .child(code(&INTERACTIVITY_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn interactivity_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-4 w-full justify-center font-mono text-xs text-woodsmoke-200")
        .children([
            ("cursor-pointer", "point at me"),
            ("cursor-grab", "grab me"),
            ("cursor-not-allowed", "forbidden"),
            ("cursor-help", "confused?"),
        ].map(|(class, label)| {
            html!("div", {
                .apply(|b| match class {
                    "cursor-pointer" => dwclass!(b, "cursor-pointer"),
                    "cursor-grab" => dwclass!(b, "cursor-grab"),
                    "cursor-not-allowed" => dwclass!(b, "cursor-not-allowed"),
                    _ => dwclass!(b, "cursor-help"),
                })
                .dwclass!("w-32 h-14 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600")
                .dwclass!("flex flex-col align-items-center justify-center gap-1")
                .child(html!("span", { .dwclass!("text-candlelight-300") .text(class) }))
                .child(html!("span", { .dwclass!("text-woodsmoke-400") .text(label) }))
            })
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-14 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600 select-none")
            .dwclass!("flex align-items-center justify-center text-woodsmoke-400")
            .text("select-none")
        }))
        .child(html!("div", {
            .dwclass!("w-32 h-14 rounded-lg bg-woodsmoke-800 border border-woodsmoke-600 select-all")
            .dwclass!("flex align-items-center justify-center text-candlelight-300")
            .text("select-all")
        }))
    })
}
