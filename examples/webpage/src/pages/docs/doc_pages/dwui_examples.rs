use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::dwui::example_small::{
    example_card_border_buttons, example_card_input, example_card_modal,
};
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

pub fn dwui_examples_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("DWUI Component Examples"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 m-b-4")
            .text("A showcase of DWUI components - a UI library built on top of dwind utilities")
        }))

        .child(doc_page_sub_header("Button Components"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 m-b-4")
            .text("Various button styles including flat, border, and disabled states")
        }))
        .child(example_box(
            example_card_border_buttons(),
            true
        ))

        .child(doc_page_sub_header("Input Components"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 m-b-4")
            .text("Form controls including text input, password, validation, slider, and select dropdown")
        }))
        .child(example_box(
            example_card_input(),
            true
        ))

        .child(doc_page_sub_header("Modal Dialog"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 m-b-4")
            .text("Interactive modal component with backdrop, close button, and escape key handling")
        }))
        .child(example_box(
            example_card_modal(),
            true
        ))
    })
}
