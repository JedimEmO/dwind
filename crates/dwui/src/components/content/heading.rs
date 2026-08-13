use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TextSize {
    Base,
    Large,
    ExtraLarge,
}

/// Semantic heading level; controls the rendered `<h1>`-`<h6>` tag so that
/// documents keep a meaningful outline for assistive technology
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    fn tag_name(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        }
    }
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
///         .level(HeadingLevel::H2)
///     }))
/// });
/// ```
#[component(render_fn = heading)]
struct Heading {
    #[signal]
    #[required]
    content: Dom,

    #[signal]
    #[default(TextSize::ExtraLarge)]
    text_size: TextSize,

    #[default(HeadingLevel::H1)]
    level: HeadingLevel,
}

pub fn heading(props: HeadingProps) -> Dom {
    let HeadingProps {
        content,
        text_size: size,
        level,
        apply,
    } = props;

    let size = size.broadcast();

    html!(level.tag_name(), {
        .dwclass_signal!("text-base", size.signal().eq(TextSize::Base))
        .dwclass_signal!("text-l", size.signal().eq(TextSize::Large))
        .dwclass_signal!("text-xl", size.signal().eq(TextSize::ExtraLarge))
        .dwclass!("w-auto font-semibold m-0")
        .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-900")
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .child_signal(content.map(Some))
    })
}
