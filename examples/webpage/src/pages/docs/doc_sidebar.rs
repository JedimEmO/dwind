use crate::pages::docs::{DocPage, DocSection};
use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwgenerate};
use dwui::heading;
use dwui::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use media_queries::breakpoint_active_signal;
use std::sync::Arc;

pub fn doc_sidebar<T>(
    doc_sections: Vec<DocSection>,
    selected_doc: impl Fn() -> T + 'static,
    goto: Arc<impl Fn(DocPage) + 'static>,
    main: impl Fn() -> Dom,
) -> impl Signal<Item = Option<Dom>>
where
    T: Signal<Item = DocPage> + 'static,
{
    let which_signal = breakpoint_active_signal(media_queries::Breakpoint::Medium);
    let show_menu = Mutable::new(false);
    let selected_doc = Arc::new(selected_doc);

    let goto = Arc::new(clone!(show_menu =>  move |v| {
        show_menu.set(false);
        goto(v)
    }));

    which_signal
        .map(clone!(show_menu => move |at_least_medium| {
            if at_least_medium {
                html!("div", {
                    .dwclass!("flex flex-row")
                    .child(doc_sidebar_inline(doc_sections.clone(), selected_doc(), goto.clone()))
                    .child(main())
                })
            } else {
                dwgenerate!("menu-text-hover", "hover:text-picton-blue-500");

                html!("div", {
                    .dwclass!("flex flex-col w-full")
                    .child(html!("div", {
                        .child(html!("h1", {
                            .dwclass!("m-l-2 text-xl font-mono font-extrabold text-picton-blue-200 cursor-pointer menu-text-hover")
                            .text_signal(selected_doc().map(|doc| format!("= {doc:?}")))
                            .event(clone!(show_menu => move |_: events::Click| {
                                show_menu.set(!show_menu.get());
                            }))
                        }))
                    }))
                    .child(main())
                    .child_signal(show_menu.signal().map(clone!(doc_sections, goto, selected_doc => move |show| {
                        if show {
                            Some(html!("div", {
                                .child(html!("div", {
                                   .dwclass!("bg-woodsmoke-950 absolute left-0 top-0 right-0 bottom-0")
                                    .style("z-index", "2")
                                    .style("opacity", "97%")
                                }))
                                .child(html!("div", {
                                   .dwclass!("absolute left-0 top-0 right-0 bottom-0")
                                    .style("z-index", "3")
                                    .child(doc_sidebar_inline(doc_sections.clone(), selected_doc(), goto.clone()))
                                }))
                            }))
                        } else {
                            None
                        }
                    })))
                })
            }
        }))
        .map(Some)
}

pub fn doc_sidebar_inline(
    doc_sections: Vec<DocSection>,
    selected_doc: impl Signal<Item = DocPage> + 'static,
    goto: Arc<impl Fn(DocPage) + 'static>,
) -> Dom {
    let selected_doc_bc = selected_doc.broadcast();

    html!("div", {
        .dwclass!("w-44 m-l-0 border-r border-woodsmoke-800 border-solid text-woodsmoke-50 h-full flex-none")
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
