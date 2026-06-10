use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;

pub fn position_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Position & Layout"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Display modes, positioning, stacking, and overflow control.")
        }))
        .child(doc_page_sub_header("Reference"))
        .child(example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                ["block / inline / inline-block / hidden".to_string(), "Display modes".to_string()],
                ["flex / inline-flex / grid / inline-grid".to_string(), "Layout containers".to_string()],
                ["relative / absolute / fixed / sticky".to_string(), "Positioning schemes".to_string()],
                ["top-0 / left-0 / …".to_string(), "Inset placement for positioned elements".to_string()],
                ["z-{0|10|20|30|40|50|auto}".to_string(), "Stacking order".to_string()],
                ["overflow-{auto|hidden|scroll|visible}".to_string(), "Overflow, plus overflow-x-/overflow-y- variants".to_string()],
            ],
        ))
        .child(doc_page_sub_header("Example"))
        .child(example_box(position_example(), false))
        .child(code(&POSITION_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn position_example() -> Dom {
    html!("div", {
        .dwclass!("relative w-full h-40 rounded-lg bg-woodsmoke-900 border border-woodsmoke-700 overflow-hidden")
        .child(html!("div", {
            .dwclass!("absolute top-0 left-0 m-2 p-l-2 p-r-2 rounded-md bg-charm-700 font-mono text-xs")
            .text("absolute top-0 left-0")
        }))
        .child(html!("div", {
            .dwclass!("absolute bottom-0 right-0 m-2 p-l-2 p-r-2 rounded-md bg-picton-blue-700 font-mono text-xs")
            .text("absolute bottom-0 right-0")
        }))
        .child(html!("div", {
            .dwclass!("absolute top-0 left-0 w-full h-full flex align-items-center justify-center")
            .child(html!("div", {
                .dwclass!("relative")
                .child(html!("div", {
                    .dwclass!("w-24 h-12 rounded-md bg-woodsmoke-700 z-10 relative flex align-items-center justify-center font-mono text-xs")
                    .text("z-10")
                }))
                .child(html!("div", {
                    .dwclass!("absolute top-0 left-0 w-24 h-12 rounded-md bg-candlelight-600 z-20 translate-x-6 translate-y-3 flex align-items-center justify-center font-mono text-xs text-woodsmoke-950")
                    .text("z-20")
                }))
            }))
        }))
    })
}
