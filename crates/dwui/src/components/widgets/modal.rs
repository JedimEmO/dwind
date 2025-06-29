use crate::theme::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::sync::Arc;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Full,
}

#[component(render_fn = modal)]
struct Modal {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(false)]
    open: bool,

    #[default(Box::new(|| {}))]
    on_close: dyn Fn() -> () + 'static,

    #[signal]
    #[default(ModalSize::Medium)]
    size: ModalSize,

    #[signal]
    #[default(true)]
    close_on_backdrop_click: bool,
}

pub fn modal(props: ModalProps) -> Dom {
    let ModalProps {
        content,
        open,
        on_close,
        size,
        close_on_backdrop_click,
        apply,
    } = props;

    let on_close = Arc::new(on_close);
    let size = size.broadcast();
    let open = open.broadcast();
    let is_open = Mutable::new(false);

    html!("div", {
        .visible_signal(open.signal())
        // Track open state
        .future(open.signal().for_each(clone!(is_open => move |open_val| {
            is_open.set(open_val);
            async {}
        })))
        // Add global escape key listener
        .global_event(clone!(on_close, is_open => move |e: events::KeyDown| {
            if is_open.get() && e.key() == "Escape" {
                (on_close)();
            }
        }))
        .style("position", "fixed")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100vw")
        .style("height", "100vh")
        .style("display", "flex")
        .style("justify-content", "center")
        .style("align-items", "center")
        .style("z-index", "50")

        // Backdrop
        .child_signal(close_on_backdrop_click.map(clone!(on_close => move |clickable| {
            Some(html!("div", {
                .style("position", "absolute")
                .style("top", "0")
                .style("left", "0")
                .style("width", "100%")
                .style("height", "100%")
                .style("background-color", "rgba(0, 0, 0, 0.5)")
                .style("z-index", "1")
                .apply_if(clickable, clone!(on_close => move |b| {
                    b.event(clone!(on_close => move |_: events::Click| {
                        (on_close)();
                    }))
                }))
            }))
        })))

        // Modal content container
        .child(html!("div", {
            .dwclass!("rounded-lg shadow-xl")
            .dwclass!("dwui-bg-void-900 dwui-text-on-primary-200")
            .dwclass!("is(.light *):dwui-bg-void-100 is(.light *):dwui-text-on-primary-800")
            .dwclass!("overflow-hidden p-6")
            .dwclass!("transition-all duration-200 ease-out")
            .style("position", "relative")
            .style("z-index", "10")
            .style("pointer-events", "auto")

            // Size classes
            .style_signal("width", size.signal().map(|s| match s {
                ModalSize::Small => "24rem",
                ModalSize::Medium => "600px",
                ModalSize::Large => "900px",
                ModalSize::Full => "90vw",
            }))
            .style_signal("height", size.signal().map(|s| match s {
                ModalSize::Full => Some("90vh"),
                _ => None,
            }))
            .style("max-width", "90vw")

            // Apply custom styles if provided
            .apply_if(apply.is_some(), move |b| {
                b.apply(apply.unwrap())
            })

            // Close button
            .child(html!("button", {
                .dwclass!("w-8 h-8 flex justify-center align-items-center rounded-full")
                .dwclass!("hover:dwui-bg-void-800 hover:dwui-text-on-primary-100")
                .dwclass!("is(.light *):hover:dwui-bg-void-200 is(.light *):hover:dwui-text-on-primary-900")
                .dwclass!("cursor-pointer transition-colors")
                .style("position", "absolute")
                .style("top", "1rem")
                .style("right", "1rem")
                .style("background", "none")
                .style("border", "none")
                .style("padding", "0")
                .style("margin", "0")
                .text("×")
                .style("font-size", "24px")
                .style("line-height", "1")
                .style("z-index", "20")
                .event(clone!(on_close => move |e: events::Click| {
                    e.stop_propagation();
                    (on_close)();
                }))
            }))

            // Modal content
            .child_signal(content)
        }))
    })
}
