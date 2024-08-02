use crate::pages::docs::doc_pages::colors::colors_page;
use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::doc_pages::responsive_design::responsive_design;
use crate::pages::docs::DocPage;
use dominator::Dom;
use futures_signals::signal::{Signal, SignalExt};
use dwind_macros::dwclass;
use dwind::prelude::*;
use crate::pages::docs::doc_pages::pseudoclass_themes::pseudo_class_themes;

pub fn doc_main_view(
    current_doc: impl Signal<Item = Option<DocPage>> + 'static,
) -> impl Signal<Item = Option<Dom>> {
    current_doc.map(|doc| {
        doc.map(|doc| match doc {
            DocPage::Flex => flex_page(),
            DocPage::Colors => colors_page(),
            DocPage::Responsiveness => responsive_design(),
            DocPage::Pseudoclasses => pseudo_class_themes(),
            _ => html!("div", {
                .dwclass!("w-full")
                .text("todo")
            }),
        })
    })
}
