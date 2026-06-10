use crate::pages::docs::{DocPage, DocSection};
use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
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
                    .dwclass!("flex flex-row w-full")
                    .child(doc_sidebar_inline(doc_sections.clone(), selected_doc(), goto.clone()))
                    .child(main())
                })
            } else {
                html!("div", {
                    .dwclass!("flex flex-col w-full")
                    .child(html!("div", {
                        .child(html!("button", {
                            .attr("type", "button")
                            .class("font-code")
                            .dwclass!("m-l-2 text-l font-bold text-candlelight-300 hover:text-candlelight-200 cursor-pointer bg-transparent border-none")
                            .text_signal(selected_doc().map(|doc| format!("☰ {doc}")))
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
                                    .style("z-index", "60")
                                    .style("opacity", "97%")
                                }))
                                .child(html!("div", {
                                   .dwclass!("absolute left-0 top-0 right-0 bottom-0 p-6")
                                    .style("z-index", "61")
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

    html!("nav", {
        .attr("aria-label", "Documentation")
        .dwclass!("w-44 m-l-0 border-r border-woodsmoke-800 border-solid text-woodsmoke-50 flex-none flex flex-col gap-6 p-t-2")
        .children(doc_sections.into_iter().map(clone!(goto => move |section| {
            let section_cloned = section.clone();
            let selected_index_signal = map_ref! {
                let selected_doc = selected_doc_bc.signal() =>  {
                    section_cloned.docs.iter().position(|v| v == selected_doc)
                }
            }.broadcast();

            html!("div", {
                .dwclass!("flex flex-col gap-2")
                .children([
                    html!("div", {
                        .class("font-code")
                        .dwclass!("text-xs text-woodsmoke-500 font-medium")
                        .text(&format!("// {}", section.title.to_lowercase()))
                    }),
                    list!({
                        .apply(|b| dwclass!(b, "flex flex-col gap-1 p-0 m-0"))
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
