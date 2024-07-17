use dominator::{html, Dom};
use dwind::box_shadow::*;
use futures_signals::signal::option;
use futures_signals_component_macro::component;

#[component(render_fn = card)]
struct Card {
    #[signal]
    content: Dom,
}

pub fn card(props: impl CardPropsTrait + 'static) -> Dom {
    let CardProps { content, apply } = props.take();

    html!("div", {
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .dwclass!("shadow-md")
        .child_signal(option(content))
    })
}
