use crate::components::widgets::modal::trap_focus;
use crate::theme::layers;
use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, with_node, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::sync::Arc;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DrawerSide {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DrawerSize {
    Small,
    Medium,
    Large,
}

/// A modal side panel sliding in from the left or right viewport edge.
///
/// Shares the modal's interaction contract: scrim backdrop (click to close,
/// configurable), Escape to close, `role="dialog"` with `aria-modal`, and a
/// Tab/Shift-Tab focus trap. The open state is controlled by the caller.
#[component(render_fn = drawer)]
struct Drawer {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(false)]
    open: bool,

    #[default(Box::new(|| {}))]
    on_close: dyn Fn() -> () + 'static,

    #[signal]
    #[default(DrawerSide::Right)]
    side: DrawerSide,

    #[signal]
    #[default(DrawerSize::Medium)]
    size: DrawerSize,

    #[signal]
    #[default(true)]
    close_on_backdrop_click: bool,

    #[signal]
    #[default("Drawer".to_string())]
    aria_label: String,
}

pub fn drawer(props: DrawerProps) -> Dom {
    let DrawerProps {
        content,
        open,
        on_close,
        side,
        size,
        close_on_backdrop_click,
        aria_label,
        apply,
    } = props;

    let on_close = Arc::new(on_close);
    let open = open.broadcast();
    let side = side.broadcast();
    let size = size.broadcast();
    let is_open = Mutable::new(false);

    // Portalled to the body so a transformed ancestor can never capture the
    // fixed positioning (see utils::body_portal).
    crate::utils::body_portal(html!("div", {
        .visible_signal(open.signal())
        .future(open.signal().for_each(clone!(is_open => move |open_val| {
            is_open.set(open_val);
            async {}
        })))
        .global_event(clone!(on_close, is_open => move |e: events::KeyDown| {
            if is_open.get() && e.key() == "Escape" {
                (on_close)();
            }
        }))
        .style("position", "fixed")
        .style("inset", "0")
        .style("z-index", layers::MODAL)

        // Backdrop
        .child_signal(close_on_backdrop_click.map(clone!(on_close => move |clickable| {
            Some(html!("div", {
                .dwclass!("dwui-scrim")
                .style("position", "absolute")
                .style("inset", "0")
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

        // Panel
        .child(html!("div", {
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("tabindex", "-1")
            .attr_signal("aria-label", aria_label)
            .focused_signal(open.signal())
            .dwclass!("dwui-bg-void-900 dwui-text-on-primary-200 shadow-2xl p-6")
            .dwclass!("is(.light *):dwui-bg-void-100 is(.light *):dwui-text-on-primary-800")
            .dwclass!("dwui-focusable")
            .style("position", "absolute")
            .style("top", "0")
            .style("bottom", "0")
            .style("overflow-y", "auto")
            .style("z-index", "10")
            .style("max-width", "90vw")
            .style_signal("width", size.signal().map(|s| match s {
                DrawerSize::Small => "20rem",
                DrawerSize::Medium => "28rem",
                DrawerSize::Large => "36rem",
            }))
            .style_signal("left", side.signal().map(|s| match s {
                DrawerSide::Left => Some("0"),
                DrawerSide::Right => None,
            }))
            .style_signal("right", side.signal().map(|s| match s {
                DrawerSide::Right => Some("0"),
                DrawerSide::Left => None,
            }))
            .style_signal("border-left", side.signal().map(|s| match s {
                DrawerSide::Right => Some("1px solid var(--dwui-void-700)"),
                DrawerSide::Left => None,
            }))
            .style_signal("border-right", side.signal().map(|s| match s {
                DrawerSide::Left => Some("1px solid var(--dwui-void-700)"),
                DrawerSide::Right => None,
            }))
            .style_signal("animation", side.signal().map(|s| match s {
                DrawerSide::Right => "dwui-slide-in-right 200ms ease-out",
                DrawerSide::Left => "dwui-slide-in-left 200ms ease-out",
            }))
            // Focus trap: Tab and Shift-Tab wrap within the drawer
            .with_node!(panel_node => {
                .event_with_options(&EventOptions::preventable(), clone!(panel_node => move |e: events::KeyDown| {
                    trap_focus(&panel_node, &e);
                }))
            })
            .apply_if(apply.is_some(), move |b| {
                b.apply(apply.unwrap())
            })

            // Close button
            .child(html!("button", {
                .attr("type", "button")
                .attr("aria-label", "Close drawer")
                .dwclass!("w-8 h-8 flex justify-center align-items-center rounded-full")
                .dwclass!("hover:dwui-bg-void-800 hover:dwui-text-on-primary-100")
                .dwclass!("is(.light *):hover:dwui-bg-void-200 is(.light *):hover:dwui-text-on-primary-900")
                .dwclass!("cursor-pointer transition-colors bg-transparent border-none p-0 m-0")
                .dwclass!("dwui-focusable")
                .style("position", "absolute")
                .style("top", "1rem")
                .style("right", "1rem")
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
                .event(clone!(on_close => move |e: events::Click| {
                    e.stop_propagation();
                    (on_close)();
                }))
            }))

            // Drawer content
            .child_signal(content)
        }))
    }))
}
