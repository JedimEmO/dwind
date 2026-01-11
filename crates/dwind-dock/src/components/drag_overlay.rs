//! Drag overlay component - visual feedback during tab dragging.

use crate::state::DockState;
use dominator::{html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;

/// Props for the drag overlay component.
pub struct DragOverlayProps {
    /// The dock state.
    pub state: DockState,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render the drag overlay - a floating preview of the dragged tab.
pub fn drag_overlay(props: DragOverlayProps) -> Dom {
    let DragOverlayProps { state, apply } = props;

    html!("div", {
        .dwclass!("fixed pointer-events-none z-50")

        // Position follows mouse with offset
        .style_signal("left", state.drag_signal().map(|d| {
            d.map(|d| format!("{}px", d.mouse_x + 10.0)).unwrap_or_default()
        }))
        .style_signal("top", state.drag_signal().map(|d| {
            d.map(|d| format!("{}px", d.mouse_y + 10.0)).unwrap_or_default()
        }))

        // Tab preview
        .child_signal(state.drag_signal().map(|drag| {
            drag.map(|d| {
                html!("div", {
                    .dwclass!("px-3 py-1 rounded shadow-lg")
                    .dwclass!("bg-blue-700 text-white")
                    .dwclass!("text-sm")
                    .style("white-space", "nowrap")
                    .text(&d.tab.title)
                })
            })
        }))

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
