use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{option, SignalExt};
use futures_signals_component_macro::component;

/// Dictates the color scheme of the component
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ColorScheme {
    Primary,
    Secondary,
    Void,
}

#[component(render_fn = card)]
struct Card {
    #[signal]
    content: Dom,

    #[signal]
    #[default(ColorScheme::Void)]
    scheme: ColorScheme,
}

dwgenerate!("card-bg-primary-light", "is(.light *):dwui-bg-primary-200");
dwgenerate!("card-bg-void-light", "is(.light *):dwui-bg-void-200");

pub fn card(props: impl CardPropsTrait + 'static) -> Dom {
    let CardProps {
        content,
        scheme,
        apply,
    } = props.take();

    let scheme = scheme.broadcast();

    html!("div", {
        .dwclass!("shadow-md rounded-lg w-full")
        .dwclass_signal!("dwui-bg-void-900 card-bg-void-light", scheme.signal().map(|v| v == ColorScheme::Void))
        .dwclass_signal!("dwui-bg-primary-900 card-bg-primary-light", scheme.signal().map(|v| v == ColorScheme::Primary))
        //.dwclass_signal!("dwui-bg-secondary-900", scheme.signal().map(|v| v == ColorScheme::Secondary))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .child_signal(option(content))
    })
}
