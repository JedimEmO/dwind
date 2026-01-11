//! Floating panel component - a draggable, resizable overlay panel.

use crate::components::dock_area::TabContentRenderer;
use crate::components::tab_panel::{tab_panel, TabPanelProps};
use crate::layout::FloatingPanel as FloatingPanelData;
use crate::state::DockState;
use dominator::{events, html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

/// Props for the floating panel component.
pub struct FloatingPanelProps {
    /// The floating panel data.
    pub panel: FloatingPanelData,
    /// The dock state.
    pub state: DockState,
    /// Function to render tab content.
    pub tab_content: TabContentRenderer,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a floating panel - a draggable overlay with tabs.
pub fn floating_panel(props: FloatingPanelProps) -> Dom {
    let FloatingPanelProps {
        panel,
        state,
        tab_content,
        apply,
    } = props;

    let position = Mutable::new((panel.x, panel.y));
    let size = Mutable::new((panel.width, panel.height));
    let is_moving = Mutable::new(false);
    let move_offset: Mutable<Option<(f64, f64)>> = Mutable::new(None);
    let is_resizing = Mutable::new(false);
    let resize_start: Mutable<Option<(f64, f64, f64, f64)>> = Mutable::new(None);

    let panel_id = panel.id;

    html!("div", {
        .dwclass!("absolute rounded shadow-lg overflow-hidden flex flex-col pointer-events-auto")
        .dwclass!("bg-gray-800 border border-gray-600")

        .style("z-index", "100")
        .style_signal("left", position.signal().map(|(x, _)| format!("{}px", x)))
        .style_signal("top", position.signal().map(|(_, y)| format!("{}px", y)))
        .style_signal("width", size.signal().map(|(w, _)| format!("{}px", w)))
        .style_signal("height", size.signal().map(|(_, h)| format!("{}px", h)))

        // Title bar for dragging
        .child(html!("div", {
            .dwclass!("h-6 cursor-move flex items-center px-2 flex-shrink-0")
            .dwclass!("bg-gray-700")

            .event({
                let is_moving = is_moving.clone();
                let move_offset = move_offset.clone();
                move |e: events::MouseDown| {
                    is_moving.set(true);
                    move_offset.set(Some((e.offset_x() as f64, e.offset_y() as f64)));
                }
            })
        }))

        // Panel content
        .child(html!("div", {
            .dwclass!("flex-1 overflow-hidden")
            .child(tab_panel(TabPanelProps {
                node_id: panel.id,
                container: panel.container.clone(),
                state: state.clone(),
                tab_content,
                apply: None,
            }))
        }))

        // Resize handle (bottom-right corner)
        .child(html!("div", {
            .dwclass!("absolute bottom-0 right-0 w-4 h-4 cursor-se-resize")
            .dwclass!("bg-gray-600 hover:bg-blue-500")

            .event({
                let is_resizing = is_resizing.clone();
                let resize_start = resize_start.clone();
                let size = size.clone();
                move |e: events::MouseDown| {
                    is_resizing.set(true);
                    let (w, h) = size.get();
                    resize_start.set(Some((e.x() as f64, e.y() as f64, w, h)));
                }
            })
        }))

        // Global events for movement
        .global_event({
            let is_moving = is_moving.clone();
            let position = position.clone();
            let move_offset = move_offset.clone();
            move |e: events::MouseMove| {
                if is_moving.get() {
                    if let Some((ox, oy)) = move_offset.get() {
                        position.set((e.x() as f64 - ox, e.y() as f64 - oy));
                    }
                }
            }
        })
        .global_event({
            let is_moving = is_moving.clone();
            let move_offset = move_offset.clone();
            let state = state.clone();
            let position = position.clone();
            move |_: events::MouseUp| {
                if is_moving.replace(false) {
                    move_offset.set(None);
                    let (x, y) = position.get();
                    state.move_floating_panel(panel_id, x, y);
                }
            }
        })

        // Global events for resizing
        .global_event({
            let is_resizing = is_resizing.clone();
            let resize_start = resize_start.clone();
            let size = size.clone();
            move |e: events::MouseMove| {
                if is_resizing.get() {
                    if let Some((start_x, start_y, start_w, start_h)) = resize_start.get() {
                        let dx = e.x() as f64 - start_x;
                        let dy = e.y() as f64 - start_y;
                        size.set((
                            (start_w + dx).max(200.0),
                            (start_h + dy).max(150.0),
                        ));
                    }
                }
            }
        })
        .global_event({
            let is_resizing = is_resizing.clone();
            let resize_start = resize_start.clone();
            let state = state.clone();
            let size = size.clone();
            move |_: events::MouseUp| {
                if is_resizing.replace(false) {
                    resize_start.set(None);
                    let (w, h) = size.get();
                    state.resize_floating_panel(panel_id, w, h);
                }
            }
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
