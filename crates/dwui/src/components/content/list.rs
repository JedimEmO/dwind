use dominator::{html, Dom};
use futures_signals_component_macro::component;
use futures_signals::signal_vec::SignalVecExt;
use dwind::prelude::*;

#[component(render_fn = pretty_list)]
struct List {
    #[signal_vec]
    #[default(vec ! [])]
    items: Dom,
}

pub fn pretty_list(props: impl ListPropsTrait + 'static) -> Dom {
    let ListProps { items, apply } = props.take();

    dwgenerate!("li-item-text", "hover:text-manatee-300");
    dwgenerate!("li-item-border", "hover:border-color-manatee-200");

    html!("ul", {
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .children_signal_vec(items.map(|item| {
            html!("li", {
                .dwclass!("li-item-border li-item-text border-l h-6 border-color-manatee-600 text-manatee-600 cursor-pointer")
                .style("padding-left","10px")
                .child(item)
            })
        }))
    })
}
