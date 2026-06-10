use crate::pages::components_page::components_page;
use crate::pages::docs::doc_pages::animations::animation_page;
use crate::pages::docs::doc_pages::borders::borders_page;
use crate::pages::docs::doc_pages::colors::colors_page;
use crate::pages::docs::doc_pages::filters::filters_page;
use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::doc_pages::getting_started::getting_started_page;
use crate::pages::docs::doc_pages::grid::grid_page;
use crate::pages::docs::doc_pages::interactivity::interactivity_page;
use crate::pages::docs::doc_pages::position::position_page;
use crate::pages::docs::doc_pages::pseudoclass_themes::pseudo_class_themes;
use crate::pages::docs::doc_pages::responsive_design::responsive_design;
use crate::pages::docs::doc_pages::shadows::shadows_page;
use crate::pages::docs::doc_pages::sizing::sizing_page;
use crate::pages::docs::doc_pages::spacing::spacing_page;
use crate::pages::docs::doc_pages::transforms::transforms_page;
use crate::pages::docs::doc_pages::typography::typography_page;
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
            DocPage::Grid => grid_page(),
            DocPage::Position => position_page(),
            DocPage::Spacing => spacing_page(),
            DocPage::Sizing => sizing_page(),
            DocPage::Typography => typography_page(),
            DocPage::Borders => borders_page(),
            DocPage::Shadows => shadows_page(),
            DocPage::Filters => filters_page(),
            DocPage::Transforms => transforms_page(),
            DocPage::Interactivity => interactivity_page(),
            DocPage::Colors => colors_page(),
            DocPage::Responsiveness => responsive_design(),
            DocPage::Pseudoclasses => pseudo_class_themes(),
            DocPage::GettingStarted => getting_started_page(),
            DocPage::DwuiExamples => components_page(),
            DocPage::Animation => animation_page(),
            _ => html!("div", {
                .dwclass!("w-full")
                .text("todo")
            }),
        })
    })
}
