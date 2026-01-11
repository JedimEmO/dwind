//! Dock area component - the root container for the dock system.

use crate::components::drag_overlay::{drag_overlay, DragOverlayProps};
use crate::components::floating_panel::{floating_panel, FloatingPanelProps};
use crate::components::split_container::{split_container, SplitContainerProps};
use crate::components::tab_panel::{tab_panel, TabPanelProps};
use crate::layout::{DockNode, TabId};
use crate::state::DockState;
use crate::theme::DockTheme;
use dominator::{html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::SignalExt;
use std::sync::Arc;
use web_sys::HtmlElement;

/// Type for the tab content renderer function.
pub type TabContentRenderer = Arc<dyn Fn(&TabId) -> Dom + Send + Sync>;

/// Props for the dock area component.
pub struct DockAreaProps {
    /// The dock state manager.
    pub state: DockState,
    /// Function to render content for a tab.
    pub tab_content: TabContentRenderer,
    /// Optional theme customization.
    pub theme: DockTheme,
    /// Optional builder function for customizing the root element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render the dock area - the root container for the dock system.
pub fn dock_area(props: DockAreaProps) -> Dom {
    let DockAreaProps {
        state,
        tab_content,
        theme: _theme,
        apply,
    } = props;

    html!("div", {
        .dwclass!("relative w-full h-full overflow-hidden bg-gray-900")

        // Render the main docked layout
        .child_signal({
            let state = state.clone();
            let tab_content = tab_content.clone();
            state.layout_signal().map(move |layout| {
                layout.root.as_ref().map(|root| {
                    render_dock_node(root, state.clone(), tab_content.clone())
                })
            })
        })

        // Render floating panels
        .child_signal({
            let state = state.clone();
            let tab_content = tab_content.clone();
            state.layout_signal().map(move |layout| {
                if layout.floating.is_empty() {
                    None
                } else {
                    Some(html!("div", {
                        .dwclass!("absolute inset-0 pointer-events-none")
                        .children(layout.floating.iter().map({
                            let state = state.clone();
                            let tab_content = tab_content.clone();
                            move |panel| {
                                floating_panel(FloatingPanelProps {
                                    panel: panel.clone(),
                                    state: state.clone(),
                                    tab_content: tab_content.clone(),
                                    apply: None,
                                })
                            }
                        }))
                    }))
                }
            })
        })

        // Drag overlay for visual feedback
        .child_signal({
            let state = state.clone();
            state.is_dragging_signal().map(move |is_dragging| {
                if is_dragging {
                    Some(drag_overlay(DragOverlayProps {
                        state: state.clone(),
                        apply: None,
                    }))
                } else {
                    None
                }
            })
        })

        // Global mouse move for drag position updates
        .global_event({
            let state = state.clone();
            move |e: dominator::events::MouseMove| {
                if state.is_dragging() {
                    state.update_drag(e.x() as f64, e.y() as f64);
                }
            }
        })
        // Global mouseup to end drag - checks pending_drop_zone set by drop zone hover
        .global_event({
            let state = state.clone();
            move |_: dominator::events::MouseUp| {
                state.end_drag();
            }
        })
        .global_event({
            let state = state.clone();
            move |e: dominator::events::KeyDown| {
                if e.key() == "Escape" {
                    state.cancel_drag();
                }
            }
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

/// Recursively render a dock node.
pub fn render_dock_node(node: &DockNode, state: DockState, tab_content: TabContentRenderer) -> Dom {
    match node {
        DockNode::Leaf { id, container } => tab_panel(TabPanelProps {
            node_id: *id,
            container: container.clone(),
            state,
            tab_content,
            apply: None,
        }),
        DockNode::Split {
            id,
            direction,
            ratio,
            first,
            second,
        } => split_container(SplitContainerProps {
            node_id: *id,
            direction: *direction,
            split_ratio: *ratio,
            first: first.as_ref().clone(),
            second: second.as_ref().clone(),
            state,
            tab_content,
            apply: None,
        }),
    }
}
