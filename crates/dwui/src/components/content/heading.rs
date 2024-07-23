use dominator::{html, Dom};
use dwind::flexbox_and_grid::*;
use dwind::sizing::*;
use dwind::typography::*;
use futures_signals::signal::{option, SignalExt};
use futures_signals_component_macro::component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TextSize {
    Base,
    Large,
    ExtraLarge,
}

/// Creates a heading component
///
/// # Example
/// ```rust,no_run
/// # use dominator::{html, text};
/// # use futures_signals::signal::always;
/// # use crate::dwui::prelude::*;
/// # use crate::dwui::heading;
/// html!("div", {
///     .child(heading!({
///         .apply(|b| b)
///         .content(text("Hi there"))
///         .content_signal(always(text("Hello there!")))
///         .text_size(TextSize::Large)
///         .text_size_signal(always(TextSize::Large))
///     }))
/// });
/// ```
#[component(render_fn = heading)]
struct Heading {
    #[signal]
    content: Dom,

    #[signal]
    #[default(TextSize::ExtraLarge)]
    text_size: TextSize,
}

pub fn heading(props: impl HeadingPropsTrait + 'static) -> Dom {
    let HeadingProps {
        content,
        text_size: size,
        apply,
    } = props.take();

    let size = size.broadcast();
    html!("div", {
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .dwclass!("w-auto font-semibold h-12 align-items-center flex")
        .child(html!("h1", {
            .dwclass_signal!("text-base", size.signal().eq(TextSize::Base))
            .dwclass_signal!("text-l", size.signal().eq(TextSize::Large))
            .dwclass_signal!("text-xl", size.signal().eq(TextSize::ExtraLarge))
            .dwclass!("w-auto font-semibold")
            .child_signal(option(content))
        }))
    })
}
