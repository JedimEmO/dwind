use crate::pages::docs::doc_pages::colors::colors_page;
use crate::pages::docs::doc_pages::dwui_examples::dwui_examples_page;
use crate::pages::docs::doc_pages::examples::examples_page;
use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::doc_pages::pseudoclass_themes::pseudo_class_themes;
use crate::pages::docs::doc_pages::responsive_design::responsive_design;
use crate::pages::docs::DocPage;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Signal, SignalExt};

pub fn doc_main_view(
    current_doc: impl Signal<Item = Option<DocPage>> + 'static,
) -> impl Signal<Item = Option<Dom>> {
    current_doc.map(|doc| {
        doc.map(|doc| match doc {
            DocPage::Flex => flex_page(),
            DocPage::Colors => colors_page(),
            DocPage::Responsiveness => responsive_design(),
            DocPage::Pseudoclasses => pseudo_class_themes(),
            DocPage::Examples => examples_page(),
            DocPage::DwuiExamples => dwui_examples_page(),
            _ => html!("div", {
                .dwclass!("w-full")
                .text("todo")
            }),
        })
    })
}
