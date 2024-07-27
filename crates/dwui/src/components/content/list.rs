use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::sync::Arc;

#[component(render_fn = pretty_list)]
struct List<TClickHandler: Fn(usize) -> () = fn(usize) -> ()> {
    #[signal_vec]
    #[default(vec ! [])]
    items: Dom,

    #[signal]
    #[default(None)]
    selected_index: Option<usize>,

    #[default(|_|{})]
    item_click_handler: TClickHandler,
}

dwgenerate!("li-item-text", "hover:text-woodsmoke-200");
dwgenerate!("li-item-border", "hover:border-woodsmoke-200");

pub fn pretty_list(props: impl ListPropsTrait + 'static) -> Dom {
    let ListProps {
        items,
        selected_index,
        item_click_handler,
        apply,
    } = props.take();
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
                .dwclass!("li-item-border li-item-text border-l h-6 border-woodsmoke-600 text-woodsmoke-400 cursor-pointer")
                .style("padding-left", "10px")
                .child(item)
                .dwclass_signal!("text-picton-blue-400", selected_signal.signal())
                .dwclass_signal!("hover:text-picton-blue-400", selected_signal.signal())
                .dwclass_signal!("font-bold", selected_signal.signal())
                .dwclass_signal!("border-picton-blue-400", selected_signal.signal())
                .apply(clone!(item_click_handler =>move |b| {
                    b.event(clone!(item_click_handler => move |_: events::Click| {
                        if let Some(idx) = index.get() {
                            item_click_handler(idx);
                        }
                    }))
                }))
            })
        })))
    })
}
