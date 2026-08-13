use crate::theme::prelude::*;
use dominator::{clone, events, html, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::sync::Arc;

#[component(render_fn = pretty_list)]
struct List {
    #[signal_vec]
    #[default(vec ! [])]
    items: Dom,

    #[signal]
    #[default(None)]
    #[into]
    selected_index: Option<usize>,

    #[default(Box::new(|_|{}))]
    item_click_handler: dyn Fn(usize) + 'static,
}

pub fn pretty_list(props: ListProps) -> Dom {
    let ListProps {
        items,
        selected_index,
        item_click_handler,
        apply,
    } = props;

    let item_click_handler = Arc::new(item_click_handler);
    let selected_index = selected_index.broadcast();

    html!("ul", {
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .children_signal_vec(items.enumerate().map(clone!(item_click_handler => move |(index, item)| {
            let selected_signal = map_ref! {
                let selected = selected_index.signal(),
                let index = index.signal() => {
                    index == selected
                }
            }.broadcast();

            html!("li", {
                .attr("tabindex", "0")
                .attr_signal("aria-current", selected_signal.signal().map(|selected| {
                    if selected { Some("true") } else { None }
                }))
                .dwclass!("border-l h-6 cursor-pointer")
                .dwclass!("dwui-border-void-600 is(.light *):dwui-border-void-300")
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                .dwclass!("hover:dwui-text-on-primary-50 is(.light *):hover:dwui-text-on-primary-950 hover:dwui-border-void-300")
                .dwclass!("transition-colors dwui-focusable p-l-2")
                .child(item)
                .dwclass_signal!("dwui-text-primary-300 is(.light *):dwui-text-primary-700", selected_signal.signal())
                .dwclass_signal!("hover:dwui-text-primary-300 is(.light *):hover:dwui-text-primary-700", selected_signal.signal())
                .dwclass_signal!("font-bold", selected_signal.signal())
                .dwclass_signal!("dwui-border-primary-400 is(.light *):dwui-border-primary-600", selected_signal.signal())
                .apply(clone!(item_click_handler, index =>move |b| {
                    b.event(clone!(item_click_handler, index => move |_: events::Click| {
                        if let Some(idx) = index.get() {
                            item_click_handler(idx);
                        }
                    }))
                    .event_with_options(&EventOptions::preventable(), clone!(item_click_handler, index => move |e: events::KeyDown| {
                        if e.key() == "Enter" || e.key() == " " {
                            e.prevent_default();

                            if let Some(idx) = index.get() {
                                item_click_handler(idx);
                            }
                        }
                    }))
                }))
            })
        })))
    })
}
