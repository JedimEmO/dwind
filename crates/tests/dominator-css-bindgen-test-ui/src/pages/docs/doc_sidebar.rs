use crate::pages::docs::{DocPage, DocSection};
use dominator::{text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::heading;
use dwui::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};

pub fn doc_sidebar(
    doc_sections: Vec<DocSection>,
    selected_doc_page: Mutable<Option<DocPage>>,
) -> Dom {
    html!("div", {
        .dwclass!("w-40 m-l-0 border-r border-color-manatee-800 border-solid text-manatee-300")
        .children(doc_sections.into_iter().map(clone!(selected_doc_page => move |section| {

            let section_cloned = section.clone();
            let selected_index_signal = map_ref! {
                let selected_doc = selected_doc_page.signal() =>  {
                    section_cloned.docs.iter().position(|v| Some(v) == selected_doc.as_ref())
                }
            }.broadcast();

            html!("div", {
                .children([
                    heading!({
                        .text_size(TextSize::Base)
                        .content(text(section.title.as_str()))
                    }),
                    list!({
                        .selected_index_signal(selected_index_signal.signal())
                        .item_click_handler(clone!(section, selected_doc_page => move |idx| {
                            selected_doc_page.set(Some(section.docs[idx]))
                        }))
                        .items(section.docs.iter().map(move |doc| {
                            text(doc.to_string().as_str())
                        }).collect::<Vec<_>>())
                    })
                ])
            })
        })))
    })
}
