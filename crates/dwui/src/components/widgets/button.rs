use dominator::{Dom, events, html};
use futures_signals_component_macro::component;
use crate::theme::prelude::*;
use dwind::prelude::*;
use dwind_macros::dwgenerate;

#[component(render_fn = button)]
struct Button<THandler: Fn(dominator::events::Click) -> () = fn(dominator::events::Click) -> ()> {
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    #[default(| _: events::Click | {})]
    click_handler: THandler,
}

dwgenerate!("bg-button-light", "is(.light *):dwui-bg-light-primary-300");
dwgenerate!("text-button-light", "is(.light *):dwui-light-text-on-primary-700");

pub fn button(props: impl ButtonPropsTrait + 'static) -> Dom {
    let ButtonProps { content, click_handler, apply } = props.take();

    html!("button", {
        .dwclass!("text-button-light bg-button-light font-bold")
        .child_signal(content)
        .event(move |e: events::Click| {
            click_handler(e);
        })
    })
}
