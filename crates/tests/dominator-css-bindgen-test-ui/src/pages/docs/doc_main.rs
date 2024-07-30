use crate::pages::docs::doc_pages::colors::colors_page;
use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::doc_pages::responsive_design::responsive_design;
use crate::pages::docs::DocPage;
use dominator::Dom;
use futures_signals::signal::{Signal, SignalExt};

pub fn doc_main_view(current_doc: impl Signal<Item = Option<DocPage>> + 'static) -> impl Signal<Item=Option<Dom>> {
    current_doc.map(|doc| {
        doc.map(|doc| {
            match doc {
                DocPage::Flex => flex_page(),
                DocPage::Colors => colors_page(),
                DocPage::Responsiveness => responsive_design(),
                _ => html!("div", { .text("todo")})
            }
        })
    })
}
