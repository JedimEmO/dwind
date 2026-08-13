use crate::theme::layers;
use crate::theme::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::sync::Arc;
use web_sys::wasm_bindgen::JsCast;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PopoverPosition {
    BottomStart,
    BottomEnd,
    TopStart,
    TopEnd,
}

/// A generic anchored overlay panel.
///
/// Wraps the `anchor` content; while `open` is set, the panel renders beside
/// it (positioned like the tooltip, relative to the anchor's box). Escape and
/// clicks outside the wrapper invoke `on_close` — the open state itself stays
/// controlled by the caller.
#[component(render_fn = popover)]
struct Popover {
    #[signal]
    #[default(None)]
    #[into]
    anchor: Option<Dom>,

    #[signal]
    #[default(None)]
    #[into]
    content: Option<Dom>,

    #[signal]
    #[default(false)]
    open: bool,

    #[default(Box::new(|| {}))]
    on_close: dyn Fn() -> () + 'static,

    #[signal]
    #[default(PopoverPosition::BottomStart)]
    position: PopoverPosition,

    #[signal]
    #[default("Popover".to_string())]
    #[into]
    aria_label: String,
}

pub fn popover(props: PopoverProps) -> Dom {
    let PopoverProps {
        anchor,
        content,
        open,
        on_close,
        position,
        aria_label,
        apply,
    } = props;

    let on_close = Arc::new(on_close);
    let open = open.broadcast();
    let position = position.broadcast();
    let is_open = Mutable::new(false);

    html!("span", {
        .dwclass!("inline-block")
        .style("position", "relative")
        .future(open.signal().for_each(clone!(is_open => move |v| {
            is_open.set(v);
            async {}
        })))
        .global_event(clone!(on_close, is_open => move |e: events::KeyDown| {
            if is_open.get() && e.key() == "Escape" {
                (on_close)();
            }
        }))
        .with_node!(wrapper => {
            // A click anywhere outside the wrapper closes the popover
            .global_event(clone!(on_close, is_open => move |e: events::Click| {
                if !is_open.get() {
                    return;
                }

                let outside = e
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .map(|node| !wrapper.contains(Some(&node)))
                    .unwrap_or(true);

                if outside {
                    (on_close)();
                }
            }))
        })
        .child_signal(anchor)
        .child(html!("div", {
            .attr_signal("aria-label", aria_label)
            .visible_signal(open.signal())
            .dwclass!("rounded-lg border shadow-lg p-2")
            .dwclass!("dwui-bg-void-800 dwui-border-void-700 dwui-text-on-primary-200")
            .dwclass!("is(.light *):dwui-bg-void-50 is(.light *):dwui-border-void-200 is(.light *):dwui-text-on-primary-800")
            .style("position", "absolute")
            .style("z-index", layers::OVERLAY)
            .style("width", "max-content")
            .style("animation", "dwui-modal-in 150ms ease-out")
            .style_signal("top", position.signal().map(|p| match p {
                PopoverPosition::BottomStart | PopoverPosition::BottomEnd => Some("calc(100% + 0.375rem)"),
                _ => None,
            }))
            .style_signal("bottom", position.signal().map(|p| match p {
                PopoverPosition::TopStart | PopoverPosition::TopEnd => Some("calc(100% + 0.375rem)"),
                _ => None,
            }))
            .style_signal("left", position.signal().map(|p| match p {
                PopoverPosition::BottomStart | PopoverPosition::TopStart => Some("0"),
                _ => None,
            }))
            .style_signal("right", position.signal().map(|p| match p {
                PopoverPosition::BottomEnd | PopoverPosition::TopEnd => Some("0"),
                _ => None,
            }))
            .child_signal(content)
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
