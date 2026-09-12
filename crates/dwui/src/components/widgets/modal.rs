use crate::theme::layers;
use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, with_node, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::sync::Arc;
use web_sys::wasm_bindgen::JsCast;

/// Keeps Tab/Shift-Tab cycling within `container`'s focusable elements.
pub(crate) fn trap_focus(container: &web_sys::HtmlElement, e: &events::KeyDown) {
    const FOCUSABLE: &str = "a[href], button:not([disabled]), input:not([disabled]), \
        select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

    if e.key() != "Tab" {
        return;
    }

    let Ok(focusables) = container.query_selector_all(FOCUSABLE) else {
        return;
    };

    let count = focusables.length();

    if count == 0 {
        e.prevent_default();
        return;
    }

    let element_at = |index: u32| -> Option<web_sys::HtmlElement> {
        focusables.item(index)?.dyn_into().ok()
    };

    let Some(first) = element_at(0) else { return };
    let Some(last) = element_at(count - 1) else {
        return;
    };

    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());

    let active_inside = active
        .as_ref()
        .map(|a| container.contains(Some(a)))
        .unwrap_or(false);

    let is_active = |element: &web_sys::HtmlElement| {
        let element: &web_sys::Element = element.as_ref();
        active.as_ref() == Some(element)
    };

    if e.shift_key() {
        if is_active(&first) || !active_inside {
            e.prevent_default();
            let _ = last.focus();
        }
    } else if is_active(&last) || !active_inside {
        e.prevent_default();
        let _ = first.focus();
    }
}

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

    /// Accessible name announced by screen readers when the dialog opens
    #[signal]
    #[default("Dialog".to_string())]
    aria_label: String,
}

pub fn modal(props: ModalProps) -> Dom {
    let ModalProps {
        content,
        open,
        on_close,
        size,
        close_on_backdrop_click,
        aria_label,
        apply,
    } = props;

    let on_close = Arc::new(on_close);
    let size = size.broadcast();
    let open = open.broadcast();
    let is_open = Mutable::new(false);

    // Portalled to the body so a transformed ancestor can never capture the
    // fixed positioning (see utils::body_portal).
    crate::utils::body_portal(html!("div", {
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
        .style("z-index", layers::MODAL)

        // Backdrop
        .child_signal(close_on_backdrop_click.map(clone!(on_close => move |clickable| {
            Some(html!("div", {
                .dwclass!("dwui-scrim")
                .style("position", "absolute")
                .style("top", "0")
                .style("left", "0")
                .style("width", "100%")
                .style("height", "100%")
                .style("backdrop-filter", "blur(4px)")
                .style("animation", "dwui-fade-in 150ms ease-out")
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
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("tabindex", "-1")
            .attr_signal("aria-label", aria_label)
            .focused_signal(open.signal())
            .dwclass!("rounded-lg shadow-2xl")
            .dwclass!("dwui-bg-void-900 dwui-text-on-primary-200")
            .dwclass!("is(.light *):dwui-bg-void-100 is(.light *):dwui-text-on-primary-800")
            .dwclass!("overflow-hidden p-6")
            .dwclass!("border dwui-border-void-700 is(.light *):dwui-border-void-200")
            .style("animation", "dwui-modal-in 200ms ease-out")
            .style("position", "relative")
            .style("z-index", "10")
            .style("pointer-events", "auto")
            .dwclass!("dwui-focusable")
            // Focus trap: Tab and Shift-Tab wrap within the dialog
            .with_node!(dialog_node => {
                .event_with_options(&EventOptions::preventable(), clone!(dialog_node => move |e: events::KeyDown| {
                    trap_focus(&dialog_node, &e);
                }))
            })

            // Size classes
            .style_signal("width", size.signal().map(|s| match s {
                ModalSize::Small => "24rem",
                ModalSize::Medium => "37.5rem",
                ModalSize::Large => "56.25rem",
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
                .attr("type", "button")
                .attr("aria-label", "Close dialog")
                .dwclass!("w-8 h-8 flex justify-center align-items-center rounded-full")
                .dwclass!("hover:dwui-bg-void-800 hover:dwui-text-on-primary-100")
                .dwclass!("is(.light *):hover:dwui-bg-void-200 is(.light *):hover:dwui-text-on-primary-900")
                .dwclass!("cursor-pointer transition-colors")
                .dwclass!("dwui-focusable")
                .style("position", "absolute")
                .style("top", "1rem")
                .style("right", "1rem")
                .dwclass!("bg-transparent border-none")
                .style("padding", "0")
                .style("margin", "0")
                .child(svg!("svg", {
                    .attr("viewBox", "0 0 14 14")
                    .attr("width", "14")
                    .attr("height", "14")
                    .attr("fill", "none")
                    .attr("aria-hidden", "true")
                    .child(svg!("path", {
                        .attr("d", "M2 2 L12 12 M12 2 L2 12")
                        .attr("stroke", "currentColor")
                        .attr("stroke-width", "1.5")
                        .attr("stroke-linecap", "round")
                    }))
                }))
                .style("z-index", "20")
                .event(clone!(on_close => move |e: events::Click| {
                    e.stop_propagation();
                    (on_close)();
                }))
            }))

            // Modal content
            .child_signal(content)
        }))
    }))
}
