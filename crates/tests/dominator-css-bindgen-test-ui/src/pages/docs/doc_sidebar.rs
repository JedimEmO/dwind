use crate::pages::docs::{DocPage, DocSection};
use dominator::{text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::heading;
use dwui::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::sync::Arc;

pub fn doc_sidebar(
    doc_sections: Vec<DocSection>,
    selected_doc: impl Signal<Item = DocPage> + 'static,
    goto: Arc<impl Fn(DocPage) -> () + 'static>,
) -> Dom {
    let selected_doc_bc = selected_doc.broadcast();

    html!("div", {
        .dwclass!("w-32 m-l-0 border-r border-woodsmoke-800 border-solid text-woodsmoke-300")
        .children(doc_sections.into_iter().map(clone!(goto => move |section| {
            let section_cloned = section.clone();
            let selected_index_signal = map_ref! {
                let selected_doc = selected_doc_bc.signal() =>  {
                    section_cloned.docs.iter().position(|v| v == selected_doc)
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
                        .item_click_handler(clone!(section, goto => move |idx| {
                            goto(section.docs[idx])
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
