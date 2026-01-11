//! Split container component for dividing space between two children.

use crate::components::dock_area::{render_dock_node, TabContentRenderer};
use crate::components::split_divider::{split_divider, SplitDividerProps};
use crate::layout::{DockNode, NodeId, SplitDirection};
use crate::state::DockState;
use dominator::{html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

/// Props for the split container component.
pub struct SplitContainerProps {
    /// The node ID for this split.
    pub node_id: NodeId,
    /// Direction of the split.
    pub direction: SplitDirection,
    /// The split ratio (0.0 to 1.0).
    pub split_ratio: f64,
    /// First child node.
    pub first: DockNode,
    /// Second child node.
    pub second: DockNode,
    /// The dock state.
    pub state: DockState,
    /// Function to render tab content.
    pub tab_content: TabContentRenderer,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a split container with resizable divider.
pub fn split_container(props: SplitContainerProps) -> Dom {
    let SplitContainerProps {
        node_id,
        direction,
        split_ratio,
        first,
        second,
        state,
        tab_content,
        apply,
    } = props;

    let ratio = Mutable::new(split_ratio);
    let is_resizing = Mutable::new(false);

    let is_horizontal = matches!(direction, SplitDirection::Horizontal);

    html!("div", {
        .dwclass!("flex w-full h-full")
        .style("flex-direction", if is_horizontal { "row" } else { "column" })

        // First child
        .child(html!("div", {
            .dwclass!("overflow-hidden")
            .style_signal("flex", ratio.signal().map(move |r| {
                format!("{} 0 0%", r)
            }))
            .child(render_dock_node(&first, state.clone(), tab_content.clone()))
        }))

        // Resizable divider
        .child(split_divider(SplitDividerProps {
            node_id,
            direction,
            ratio: ratio.clone(),
            is_resizing: is_resizing.clone(),
            state: state.clone(),
            apply: None,
        }))

        // Second child
        .child(html!("div", {
            .dwclass!("overflow-hidden")
            .style_signal("flex", ratio.signal().map(move |r| {
                format!("{} 0 0%", 1.0 - r)
            }))
            .child(render_dock_node(&second, state.clone(), tab_content.clone()))
        }))

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
