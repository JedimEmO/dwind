use crate::my_custom_theme::HOVER_BG_APPLE;
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
use wasm_bindgen_futures::spawn_local;

pub fn doc_sidebar<T>(
    doc_sections: Vec<DocSection>,
    selected_doc: impl Fn() -> T + 'static,
    goto: Arc<impl Fn(DocPage) + 'static>,
    main: impl Fn() -> Dom,
) -> impl Signal<Item = Option<Dom>>
where
    T: Signal<Item = DocPage> + 'static,
{
    let which_signal = breakpoint_active_signal(media_queries::Breakpoint::Small);
    let show_small_menu = Mutable::new(false);
    let selected_doc = Arc::new(selected_doc);

    let goto = Arc::new(clone!(show_small_menu =>  move |v| {
        show_small_menu.set(false);
        goto(v)
    }));

    which_signal
        .map(clone!(show_small_menu => move |at_least_small| {
            if at_least_small {
                html!("div", {
                    .dwclass!("grid grid-cols-2 place-content-start")
                    .child(doc_sidebar_inline(doc_sections.clone(), selected_doc(), goto.clone()))
                    .child(main())
                })
            } else {
                dwgenerate!("menu-text-hover", "hover:text-picton-blue-500");

                html!("div", {
                    .class([
                        &*GRID,
                        &*GRID_FLOW_ROW
                    ])
                    .child(html!("div", {
                        .child(html!("h1", {
                            .class([&*M_L_2, &*TEXT_XL, &*FONT_MONO, &*FONT_EXTRABOLD, &*TEXT_PICTON_BLUE_200, &*CURSOR_POINTER, &*MENU_TEXT_HOVER])
                            .text("=")
                            .event(clone!(show_small_menu => move |_: events::Click| {
                                show_small_menu.set(!show_small_menu.get());
                            }))
                        }))
                    }))
                    .child(html!("div", {
                        .class([&*COL_START_1, &*COL_END_2, &*ROW_START_2, &*ROW_END_2])
                        .child(main())
                    }))
                    .child_signal(show_small_menu.signal().map(clone!(doc_sections, goto, selected_doc => move |show| {
                        if show {
                            Some(html!("div", {
                                .class([&*COL_START_1, &*COL_END_2, &*ROW_START_2, &*ROW_END_2, &*BG_WOODSMOKE_950])
                                .style("z-index", "2")
                                .child(doc_sidebar_inline(doc_sections.clone(), selected_doc(), goto.clone()))
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
        .dwclass!("w-40 m-l-0 border-r border-woodsmoke-800 border-solid text-woodsmoke-300")
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
