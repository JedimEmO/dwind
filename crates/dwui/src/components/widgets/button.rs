use crate::theme::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use dwind_macros::dwgenerate;
use futures_signals_component_macro::component;

#[component(render_fn = button)]
struct Button<THandler: Fn(events::Click) -> () = fn(events::Click) -> ()> {
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    #[default(| _: events::Click | {})]
    click_handler: THandler,
}

pub fn button(props: impl ButtonPropsTrait + 'static) -> Dom {
    let ButtonProps {
        content,
        click_handler,
        apply,
    } = props.take();

    dwgenerate!("button-bg-light", "is(.light *):dwui-bg-primary-400");
    dwgenerate!(
        "button-bg-hover-light",
        "is(.light *):hover:dwui-bg-primary-600"
    );
    dwgenerate!("button-text-light", "is(.light *):dwui-text-on-primary-700");

    html!("button", {
        .dwclass!("button-text-light button-bg-light button-bg-hover-light hover:dwui-bg-primary-800 dwui-bg-primary-700 dwui-text-on-primary-200")
        .dwclass!("w-full font-bold p-2 cursor-pointer rounded-lg")
        .child_signal(content)
        .event(move |e: events::Click| {
            click_handler(e);
        })
    })
}
