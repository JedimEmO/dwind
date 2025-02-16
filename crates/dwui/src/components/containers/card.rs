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

pub fn card(props: CardProps) -> Dom {
    let CardProps {
        content,
        scheme,
        apply,
    } = props;

    let scheme = scheme.broadcast();

    html!("div", {
        .dwclass!("rounded w-full")
        .dwclass_signal!("dwui-bg-void-900 dwui-text-on-primary-200", scheme.signal().map(|v| v == ColorScheme::Void))
        .dwclass_signal!("is(.light *):dwui-bg-void-400 is(.light *):dwui-text-on-primary-800", scheme.signal().map(|v| v == ColorScheme::Void))
        .dwclass_signal!("dwui-bg-primary-900", scheme.signal().map(|v| v == ColorScheme::Primary))
        //.dwclass_signal!("dwui-bg-secondary-900", scheme.signal().map(|v| v == ColorScheme::Secondary))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .child_signal(option(content))
    })
}
