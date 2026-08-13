use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

/// Dictates the color scheme of the component
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ColorScheme {
    Primary,
    Void,
}

/// Content padding of a [`card`]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardPadding {
    None,
    Small,
    Medium,
    Large,
}

#[component(render_fn = card)]
struct Card {
    #[signal]
    #[required]
    content: Dom,

    #[signal]
    #[default(ColorScheme::Void)]
    scheme: ColorScheme,

    #[signal]
    #[default(CardPadding::Medium)]
    padding: CardPadding,
}

pub fn card(props: CardProps) -> Dom {
    let CardProps {
        content,
        scheme,
        padding,
        apply,
    } = props;

    let scheme = scheme.broadcast();
    let padding = padding.broadcast();

    html!("div", {
        .dwclass!("rounded-lg w-full shadow-lg")
        .dwclass!("border dwui-border-void-800 is(.light *):dwui-border-void-300")
        .dwclass_signal!("dwui-bg-void-900 dwui-text-on-primary-200", scheme.signal().map(|v| v == ColorScheme::Void))
        .dwclass_signal!("is(.light *):dwui-bg-void-200 is(.light *):dwui-text-on-primary-800", scheme.signal().map(|v| v == ColorScheme::Void))
        .dwclass_signal!("dwui-bg-primary-900 dwui-text-on-primary-100 dwui-border-primary-800", scheme.signal().map(|v| v == ColorScheme::Primary))
        .dwclass_signal!("is(.light *):dwui-bg-primary-100 is(.light *):dwui-text-on-primary-900 is(.light *):dwui-border-primary-300", scheme.signal().map(|v| v == ColorScheme::Primary))
        .dwclass_signal!("p-2", padding.signal().map(|v| v == CardPadding::Small))
        .dwclass_signal!("p-4", padding.signal().map(|v| v == CardPadding::Medium))
        .dwclass_signal!("p-6", padding.signal().map(|v| v == CardPadding::Large))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .child_signal(content.map(Some))
    })
}
