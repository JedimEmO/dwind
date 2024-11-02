use crate::theme::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use dwind_macros::dwgenerate;
use futures_signals::signal::{SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonType {
    Flat,
    Border,
}

#[component(render_fn = button)]
struct Button<THandler: Fn(events::Click) -> () = fn(events::Click) -> ()> {
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    #[default(| _: events::Click | {})]
    click_handler: THandler,
    #[signal]
    #[default(false)]
    disabled: bool,
    #[signal]
    #[default(ButtonType::Flat)]
    button_type: ButtonType,
}

dwgenerate!("button-bg-light", "is(.light *):dwui-bg-primary-400");
dwgenerate!(
    "button-border-light",
    "is(.light *):border-dwui-primary-400"
);
dwgenerate!(
    "button-bg-light-disabled",
    "is(.light *):disabled:dwui-bg-primary-700"
);
dwgenerate!(
    "button-border-light-disabled",
    "is(.light *):disabled:border-dwui-primary-700"
);
dwgenerate!(
    "button-bg-hover-light",
    "is(.light *):hover:dwui-bg-primary-600"
);
dwgenerate!(
    "button-border-hover-light",
    "is(.light *):hover:border-dwui-primary-800"
);
dwgenerate!("button-text-light", "is(.light *):dwui-text-on-primary-700");
dwgenerate!(
    "button-text-light-disabled",
    "is(.light *):disabled:dwui-text-on-primary-800"
);
dwgenerate!(
    "button-border-text-light",
    "is(.light *):dwui-text-on-primary-700"
);

pub fn button(props: impl ButtonPropsTrait + 'static) -> Dom {
    let ButtonProps {
        content,
        click_handler,
        disabled,
        button_type,
        apply,
    } = props.take();

    let button_type = button_type.broadcast();

    html!("button", {
        .dwclass_signal!("button-text-light button-bg-light button-bg-hover-light hover:dwui-bg-primary-800 dwui-bg-primary-700 button-bg-light-disabled disabled:dwui-bg-primary-900 button-text-light-disabled", button_type.signal().map(|v| v == ButtonType::Flat))
        .dwclass_signal!("button-border-text-light border-dwui-primary-500 bg-unset border button-border-light button-border-light-disabled disabled:border-dwui-primary-900 button-border-hover-light hover:border-dwui-primary-950", button_type.signal().map(|v| v == ButtonType::Border))
        .dwclass!("disabled:dwui-text-on-primary-500 dwui-text-on-primary-200")
        .dwclass!("w-full font-bold p-2 cursor-pointer rounded-lg")
        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
        .attr_signal("button-type", button_type.signal().map(|v| if v == ButtonType::Border { "bordered" } else { "flat" }))
        .attr_signal("disabled", disabled.map(|v| if v { Some("disabled") } else { None }))
        .child_signal(content)
        .event(move |e: events::Click| {
            click_handler(e);
        })
    })
}
