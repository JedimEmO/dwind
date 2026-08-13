use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BadgeVariant {
    Primary,
    Error,
    Void,
    Outline,
}

/// A small status pill for labels, counts, and state indicators.
#[component(render_fn = badge)]
struct Badge {
    #[signal]
    #[default(None)]
    #[into]
    content: Option<Dom>,

    #[signal]
    #[default(BadgeVariant::Primary)]
    variant: BadgeVariant,
}

pub fn badge(props: BadgeProps) -> Dom {
    let BadgeProps {
        content,
        variant,
        apply,
    } = props;

    let variant = variant.broadcast();

    html!("span", {
        .dwclass!("inline-flex align-items-center justify-center rounded-full")
        .dwclass!("text-xs font-bold h-5 p-l-2 p-r-2 select-none w-fit")
        .dwclass_signal!("dwui-bg-primary-700 dwui-text-on-primary-100 is(.light *):dwui-bg-primary-300 is(.light *):dwui-text-on-primary-900", variant.signal().map(|v| v == BadgeVariant::Primary))
        .dwclass_signal!("dwui-bg-error-700 is(.light *):dwui-bg-error-300", variant.signal().map(|v| v == BadgeVariant::Error))
        .dwclass_signal!("dwui-bg-void-700 dwui-text-on-primary-200 is(.light *):dwui-bg-void-300 is(.light *):dwui-text-on-primary-900", variant.signal().map(|v| v == BadgeVariant::Void))
        .dwclass_signal!("border dwui-border-primary-500 bg-transparent is(.light *):dwui-border-primary-400", variant.signal().map(|v| v == BadgeVariant::Outline))
        .dwclass_signal!("dwui-text-primary-300 is(.light *):dwui-text-primary-700", variant.signal().map(|v| v == BadgeVariant::Outline))
        .dwclass_signal!("dwui-text-on-accent", variant.signal().map(|v| v == BadgeVariant::Error))
        .child_signal(content)
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
