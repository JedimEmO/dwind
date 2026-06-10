use crate::theme::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonType {
    /// Solid gradient background
    Flat,
    /// Transparent with a visible border
    Border,
    /// Borderless text button for low-emphasis actions
    Text,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component(render_fn = button)]
struct Button {
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    #[default(Box::new(| _: events::Click | {}))]
    on_click: dyn Fn(events::Click) -> () + 'static,
    #[signal]
    #[default(false)]
    disabled: bool,
    #[signal]
    #[default(ButtonType::Flat)]
    button_type: ButtonType,
    #[signal]
    #[default(ButtonSize::Medium)]
    size: ButtonSize,
}

pub fn button(props: ButtonProps) -> Dom {
    let ButtonProps {
        content,
        on_click,
        disabled,
        button_type,
        size,
        apply,
    } = props;

    let button_type = button_type.broadcast();
    let disabled = disabled.broadcast();
    let size = size.broadcast();

    html!("button", {
        .attr("type", "button")
        .dwclass!("is(.light *):dwui-text-on-primary-900 dwui-text-on-primary-50 hover:brightness-110 active:brightness-90")
        .dwclass_signal!("linear-gradient-90 dwui-gradient-from-primary-800 dwui-gradient-to-primary-900", button_type.signal().map(|v| v == ButtonType::Flat))
        .dwclass_signal!("is(.light *):dwui-gradient-from-primary-400 is(.light *):dwui-gradient-to-primary-500", button_type.signal().map(|v| v == ButtonType::Flat))
        .dwclass_signal!("dwui-border-primary-700 hover:dwui-border-primary-600 border bg-unset", button_type.signal().map(|v| v == ButtonType::Border))
        .dwclass_signal!("is(.light *):dwui-border-primary-200 is(.light *):hover:dwui-border-primary-300 border bg-unset", button_type.signal().map(|v| v == ButtonType::Border))
        .dwclass_signal!("bg-unset border-none dwui-text-primary-300 hover:dwui-text-primary-200 is(.light *):dwui-text-primary-700 is(.light *):hover:dwui-text-primary-800", button_type.signal().map(|v| v == ButtonType::Text))
        .dwclass_signal!("h-8 text-sm", size.signal().map(|v| v == ButtonSize::Small))
        .dwclass_signal!("h-10 text-base", size.signal().map(|v| v == ButtonSize::Medium))
        .dwclass_signal!("h-12 text-l", size.signal().map(|v| v == ButtonSize::Large))
        .dwclass!("disabled:dwui-text-on-primary-500 disabled:hover:dwui-border-primary-800 disabled:cursor-not-allowed disabled:opacity-60 disabled:hover:brightness-100")
        .dwclass!("is(.light *):disabled:dwui-text-on-primary-600 is(.light *):disabled:hover:dwui-border-primary-200")
        .dwclass!("w-full font-bold p-l-3 p-r-3 cursor-pointer rounded-full pointer-events-auto")
        .dwclass!("transition-all duration-150")
        .dwclass!("focus-visible:ring-2 focus-visible:dwui-ring-primary-400 is(.light *):focus-visible:dwui-ring-primary-600")
        .style("outline", "none")
        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
        .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
        .child_signal(content)
        .event(move |e: events::Click| {
            (on_click)(e);
        })
    })
}
