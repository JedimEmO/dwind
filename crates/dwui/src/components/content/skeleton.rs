use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SkeletonVariant {
    /// A single line of text
    Text,
    /// A circle (e.g. avatar placeholder)
    Circle,
    /// A rectangular block (e.g. image or card placeholder)
    Rect,
}

/// A pulsing loading placeholder. Hidden from assistive technology;
/// pair with a [`crate::components::widgets::spinner`] or live region for announcements.
#[component(render_fn = skeleton)]
struct Skeleton {
    #[signal]
    #[default(SkeletonVariant::Text)]
    variant: SkeletonVariant,
}

pub fn skeleton(props: SkeletonProps) -> Dom {
    let SkeletonProps { variant, apply } = props;

    let variant = variant.broadcast();

    html!("div", {
        .attr("aria-hidden", "true")
        .dwclass!("dwui-bg-void-700 is(.light *):dwui-bg-void-300")
        .dwclass_signal!("rounded-sm h-4 w-full", variant.signal().map(|v| v == SkeletonVariant::Text))
        .dwclass_signal!("rounded-full w-10 h-10 flex-none", variant.signal().map(|v| v == SkeletonVariant::Circle))
        .dwclass_signal!("rounded-md w-full h-24", variant.signal().map(|v| v == SkeletonVariant::Rect))
        .style("animation", "dwui-skeleton-pulse 1.6s ease-in-out infinite")
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
