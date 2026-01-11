//! Tab panel component - a container with a tab bar and content area.

use crate::components::dock_area::TabContentRenderer;
use crate::components::drop_zone_overlay::{drop_zone_overlay, DropZoneOverlayProps};
use crate::components::tab_bar::{tab_bar, TabBarProps};
use crate::layout::{NodeId, TabContainer};
use crate::state::DockState;
use dominator::{html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

/// Props for the tab panel component.
pub struct TabPanelProps {
    /// The node ID for this panel.
    pub node_id: NodeId,
    /// The tab container with tabs and active index.
    pub container: TabContainer,
    /// The dock state.
    pub state: DockState,
    /// Function to render tab content.
    pub tab_content: TabContentRenderer,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a tab panel with tab bar and content.
pub fn tab_panel(props: TabPanelProps) -> Dom {
    let TabPanelProps {
        node_id,
        container,
        state,
        tab_content,
        apply,
    } = props;

    let active_index = Mutable::new(container.active_tab_index);
    let tabs = container.tabs.clone();

    html!("div", {
        .dwclass!("flex flex-col h-full w-full relative")
        .dwclass!("bg-gray-900")

        // Tab bar
        .child(tab_bar(TabBarProps {
            node_id,
            tabs: tabs.clone(),
            active_index: active_index.clone(),
            state: state.clone(),
            apply: None,
        }))

        // Tab content area
        .child(html!("div", {
            .dwclass!("flex-1 overflow-auto")
            .child_signal({
                let tabs = tabs.clone();
                let tab_content = tab_content.clone();
                active_index.signal().map(move |idx| {
                    tabs.get(idx).map(|tab| {
                        (tab_content)(&tab.id)
                    })
                })
            })
        }))

        // Drop zone indicators (shown during drag)
        .child_signal({
            let state = state.clone();
            state.is_dragging_signal().map(move |dragging| {
                if dragging {
                    Some(drop_zone_overlay(DropZoneOverlayProps {
                        node_id,
                        state: state.clone(),
                        apply: None,
                    }))
                } else {
                    None
                }
            })
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
