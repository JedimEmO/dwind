//! Tab bar component for displaying and interacting with tabs.

use crate::layout::{NodeId, Tab};
use crate::state::DockState;
use dominator::{events, html, Dom, DomBuilder, EventOptions};
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use web_sys::HtmlElement;

/// Props for the tab bar component.
pub struct TabBarProps {
    /// The node ID this tab bar belongs to.
    pub node_id: NodeId,
    /// The tabs to display.
    pub tabs: Vec<Tab>,
    /// The currently active tab index.
    pub active_index: Mutable<usize>,
    /// The dock state for drag operations.
    pub state: DockState,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a tab bar with multiple tab headers.
pub fn tab_bar(props: TabBarProps) -> Dom {
    let TabBarProps {
        node_id,
        tabs,
        active_index,
        state,
        apply,
    } = props;

    html!("div", {
        .dwclass!("flex flex-row h-8 overflow-x-auto flex-shrink-0")
        .dwclass!("bg-gray-800 border-b border-gray-700")

        .children(tabs.into_iter().enumerate().map({
            let active_index = active_index.clone();
            let state = state.clone();
            move |(index, tab)| {
                let active_index = active_index.clone();
                let state = state.clone();
                tab_header(TabHeaderProps {
                    tab,
                    index,
                    node_id,
                    is_active: Box::new(active_index.signal().map(move |active| active == index)),
                    state: state.clone(),
                    on_click: {
                        let active_index = active_index.clone();
                        let state = state.clone();
                        Box::new(move || {
                            active_index.set(index);
                            state.set_active_tab(node_id, index);
                        })
                    },
                    apply: None,
                })
            }
        }))

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

/// Props for a single tab header.
pub struct TabHeaderProps {
    /// The tab data.
    pub tab: Tab,
    /// Index of this tab in the container.
    pub index: usize,
    /// The node ID this tab belongs to.
    pub node_id: NodeId,
    /// Signal indicating if this tab is active.
    pub is_active: Box<dyn Signal<Item = bool> + Unpin>,
    /// The dock state for drag operations.
    pub state: DockState,
    /// Callback when the tab is clicked.
    pub on_click: Box<dyn Fn() + 'static>,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a single tab header.
pub fn tab_header(props: TabHeaderProps) -> Dom {
    let TabHeaderProps {
        tab,
        index: _,
        node_id,
        is_active,
        state,
        on_click,
        apply,
    } = props;

    let is_active = is_active.broadcast();
    let is_dragging = Mutable::new(false);
    let drag_start_pos: Mutable<Option<(i32, i32)>> = Mutable::new(None);

    html!("div", {
        .dwclass!("flex flex-row items-center px-3 py-1 cursor-pointer select-none")
        .dwclass!("transition-colors duration-150")
        .dwclass!("hover:bg-gray-700")

        // Active tab styling
        .dwclass_signal!("bg-gray-900 text-gray-100", is_active.signal())
        .dwclass_signal!("text-gray-400", is_active.signal().map(|a| !a))

        // Tab title
        .child(html!("span", {
            .dwclass!("text-sm")
            .style("white-space", "nowrap")
            .text(&tab.title)
        }))

        // Close button (if closable)
        // NOTE: We handle close on mousedown, not click, because the parent's
        // mousedown triggers drag detection which can cause a re-render,
        // destroying the button element before the click event fires.
        .apply_if(tab.closable, {
            let tab = tab.clone();
            let state = state.clone();
            move |b: DomBuilder<HtmlElement>| {
                b.child(html!("span", {
                    .dwclass!("ml-2 px-1 rounded")
                    .dwclass!("hover:bg-red-600 hover:text-white cursor-pointer")
                    .dwclass!("text-gray-400 text-sm font-bold")
                    .text("×")
                    .event_with_options(&EventOptions::preventable(), {
                        let tab = tab.clone();
                        let state = state.clone();
                        move |e: events::MouseDown| {
                            e.stop_propagation();
                            e.prevent_default();
                            state.close_tab(node_id, &tab.id);
                        }
                    })
                    // Prevent click from bubbling to parent (tab activation)
                    .event(move |e: events::Click| {
                        e.stop_propagation();
                    })
                }))
            }
        })

        // Click to activate
        .event(move |_: events::Click| {
            (on_click)();
        })

        // Drag handling - mouse down starts potential drag
        .event({
            let drag_start_pos = drag_start_pos.clone();
            move |e: events::MouseDown| {
                drag_start_pos.set(Some((e.x(), e.y())));
            }
        })

        // Global mouse move to detect drag threshold
        .global_event({
            let tab = tab.clone();
            let drag_start_pos = drag_start_pos.clone();
            let is_dragging = is_dragging.clone();
            let state = state.clone();
            move |e: events::MouseMove| {
                if let Some((start_x, start_y)) = drag_start_pos.get() {
                    let dx = (e.x() - start_x).abs();
                    let dy = (e.y() - start_y).abs();

                    // Start drag after 5px threshold
                    if !is_dragging.get() && (dx > 5 || dy > 5) {
                        is_dragging.set(true);
                        state.start_drag(tab.clone(), node_id, e.x() as f64, e.y() as f64);
                    }
                }
            }
        })

        // Global mouse up ends drag detection
        .global_event({
            let drag_start_pos = drag_start_pos.clone();
            let is_dragging = is_dragging.clone();
            move |_: events::MouseUp| {
                drag_start_pos.set(None);
                is_dragging.set(false);
            }
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
