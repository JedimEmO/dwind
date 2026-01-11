//! Split divider component for resizing split panes.

use crate::layout::{NodeId, SplitDirection};
use crate::state::DockState;
use dominator::{class, events, html, with_node, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::Mutable;
use web_sys::HtmlElement;

/// Props for the split divider component.
pub struct SplitDividerProps {
    /// The node ID for this split.
    pub node_id: NodeId,
    /// Direction of the split.
    pub direction: SplitDirection,
    /// The current split ratio.
    pub ratio: Mutable<f64>,
    /// Whether the divider is currently being resized.
    pub is_resizing: Mutable<bool>,
    /// The dock state.
    pub state: DockState,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a resizable split divider.
pub fn split_divider(props: SplitDividerProps) -> Dom {
    let SplitDividerProps {
        node_id,
        direction,
        ratio,
        is_resizing,
        state,
        apply,
    } = props;

    let is_horizontal = matches!(direction, SplitDirection::Horizontal);

    html!("div", {
        .dwclass!("flex-shrink-0 bg-gray-700 hover:bg-blue-500 transition-colors")
        .class_signal(class! { .style("background-color", "rgb(59, 130, 246)") }, is_resizing.signal())
        // Set size based on direction using inline styles
        .style("width", if is_horizontal { "4px" } else { "100%" })
        .style("height", if is_horizontal { "100%" } else { "4px" })
        .style("cursor", if is_horizontal { "col-resize" } else { "row-resize" })

        .with_node!(divider_el => {
            .event({
                let is_resizing = is_resizing.clone();
                move |_: events::MouseDown| {
                    is_resizing.set(true);
                }
            })

            .global_event({
                let is_resizing = is_resizing.clone();
                let ratio = ratio.clone();
                let divider_el = divider_el.clone();
                move |e: events::MouseMove| {
                    if is_resizing.get() {
                        // Calculate new ratio based on mouse position relative to parent
                        if let Some(parent) = divider_el.parent_element() {
                            let rect = parent.get_bounding_client_rect();
                            let new_ratio = match direction {
                                SplitDirection::Horizontal => {
                                    (e.x() as f64 - rect.left()) / rect.width()
                                }
                                SplitDirection::Vertical => {
                                    (e.y() as f64 - rect.top()) / rect.height()
                                }
                            };
                            let clamped = new_ratio.clamp(0.1, 0.9);
                            ratio.set(clamped);
                        }
                    }
                }
            })

            .global_event({
                let is_resizing = is_resizing.clone();
                let ratio = ratio.clone();
                let state = state.clone();
                move |_: events::MouseUp| {
                    if is_resizing.replace(false) {
                        // Persist the new ratio
                        state.set_split_ratio(node_id, ratio.get());
                    }
                }
            })
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
