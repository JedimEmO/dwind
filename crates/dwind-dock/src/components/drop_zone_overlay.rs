//! Drop zone overlay component - visual indicators for valid drop targets.

use crate::drop_zone::{DockSide, DropZone};
use crate::layout::NodeId;
use crate::state::DockState;
use dominator::{events, html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

/// Props for the drop zone overlay component.
pub struct DropZoneOverlayProps {
    /// The node ID this overlay is for.
    pub node_id: NodeId,
    /// The dock state.
    pub state: DockState,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render drop zone overlay with indicators for each drop location.
pub fn drop_zone_overlay(props: DropZoneOverlayProps) -> Dom {
    let DropZoneOverlayProps {
        node_id,
        state,
        apply,
    } = props;

    let hovered_side: Mutable<Option<DockSide>> = Mutable::new(None);

    html!("div", {
        .dwclass!("absolute inset-0 pointer-events-none")
        .style("z-index", "9998")

        // Center drop zone (for tab bar)
        .child(drop_zone_indicator(DropZoneIndicatorProps {
            side: DockSide::Center,
            node_id,
            hovered_side: hovered_side.clone(),
            state: state.clone(),
            apply: None,
        }))

        // Edge drop zones
        .child(drop_zone_indicator(DropZoneIndicatorProps {
            side: DockSide::Top,
            node_id,
            hovered_side: hovered_side.clone(),
            state: state.clone(),
            apply: None,
        }))
        .child(drop_zone_indicator(DropZoneIndicatorProps {
            side: DockSide::Bottom,
            node_id,
            hovered_side: hovered_side.clone(),
            state: state.clone(),
            apply: None,
        }))
        .child(drop_zone_indicator(DropZoneIndicatorProps {
            side: DockSide::Left,
            node_id,
            hovered_side: hovered_side.clone(),
            state: state.clone(),
            apply: None,
        }))
        .child(drop_zone_indicator(DropZoneIndicatorProps {
            side: DockSide::Right,
            node_id,
            hovered_side: hovered_side.clone(),
            state: state.clone(),
            apply: None,
        }))

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

/// Props for a single drop zone indicator.
pub struct DropZoneIndicatorProps {
    /// Which side this indicator is for.
    pub side: DockSide,
    /// The node ID.
    pub node_id: NodeId,
    /// Shared state for which side is hovered.
    pub hovered_side: Mutable<Option<DockSide>>,
    /// The dock state.
    pub state: DockState,
    /// Optional builder function for customizing the element.
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

/// Render a single drop zone indicator.
pub fn drop_zone_indicator(props: DropZoneIndicatorProps) -> Dom {
    let DropZoneIndicatorProps {
        side,
        node_id,
        hovered_side,
        state,
        apply,
    } = props;

    // Position styling based on side - using inline styles to avoid dwclass! variable issue
    let (top, bottom, left, right, width, height) = match side {
        DockSide::Top => ("0", "auto", "25%", "25%", "50%", "25%"),
        DockSide::Bottom => ("auto", "0", "25%", "25%", "50%", "25%"),
        DockSide::Left => ("25%", "25%", "0", "auto", "25%", "50%"),
        DockSide::Right => ("25%", "25%", "auto", "0", "25%", "50%"),
        DockSide::Center => ("25%", "25%", "25%", "25%", "50%", "50%"),
    };

    html!("div", {
        .dwclass!("absolute pointer-events-auto rounded")
        .style("z-index", "9999")

        // Position via inline styles
        .style("top", top)
        .style("bottom", bottom)
        .style("left", left)
        .style("right", right)
        .style("width", width)
        .style("height", height)

        // Always show a subtle indicator, highlight on hover
        .style("border", "2px dashed rgba(59, 130, 246, 0.5)")
        .style("background-color", "rgba(59, 130, 246, 0.1)")
        .style("transition", "all 0.15s ease")

        // Highlight on hover (local state only, no DockState updates)
        .style_signal("background-color", {
            let hovered_side = hovered_side.clone();
            hovered_side.signal().map(move |h| {
                if h == Some(side) {
                    "rgba(59, 130, 246, 0.4)"
                } else {
                    "rgba(59, 130, 246, 0.1)"
                }
            })
        })
        .style_signal("border-color", {
            let hovered_side = hovered_side.clone();
            hovered_side.signal().map(move |h| {
                if h == Some(side) {
                    "rgba(59, 130, 246, 0.9)"
                } else {
                    "rgba(59, 130, 246, 0.5)"
                }
            })
        })

        // Track hover state - update both local state and DockState's pending zone
        .event({
            let hovered_side = hovered_side.clone();
            let state = state.clone();
            move |_: events::MouseEnter| {
                hovered_side.set(Some(side));
                // Set pending drop zone (uses RefCell, no signal updates = no lag)
                let zone = match side {
                    DockSide::Center => DropZone::tab_bar(node_id),
                    _ => DropZone::split(node_id, side),
                };
                state.set_pending_drop_zone(Some(zone));
            }
        })
        .event({
            let hovered_side = hovered_side.clone();
            let state = state.clone();
            move |_: events::MouseLeave| {
                hovered_side.set(None);
                state.set_pending_drop_zone(None);
            }
        })

        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
