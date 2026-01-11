//! # dwind-dock
//!
//! A dockable views component library for DOMINATOR - Blender-style panel system.
//!
//! This crate provides a flexible docking system with:
//! - Split panes (horizontal and vertical)
//! - Tabbed panels
//! - Drag-and-drop tab reordering and docking
//! - Floating panels
//! - Layout persistence via serde
//!
//! ## Example
//!
//! ```ignore
//! use dwind_dock::prelude::*;
//! use std::sync::Arc;
//!
//! // Create initial layout
//! let layout = DockLayout::new(
//!     DockNode::hsplit(
//!         DockNode::leaf(Tab::new("files", "Files")),
//!         DockNode::leaf(Tab::new("editor", "Editor")),
//!         0.25,
//!     )
//! );
//!
//! // Create state with persistence callback
//! let state = DockState::new(layout, |layout| {
//!     // Save layout to localStorage, etc.
//! });
//!
//! // Render the dock
//! let tab_content: TabContentRenderer = Arc::new(|tab_id| {
//!     match tab_id.as_str() {
//!         "files" => html!("div", { .text("Files") }),
//!         "editor" => html!("div", { .text("Editor") }),
//!         _ => html!("div", { .text("Unknown") }),
//!     }
//! });
//!
//! dock_area(DockAreaProps {
//!     state,
//!     tab_content,
//!     theme: Default::default(),
//!     apply: None,
//! });
//! ```

pub mod components;
pub mod drop_zone;
pub mod layout;
pub mod prelude;
pub mod state;
pub mod theme;

// Re-export commonly used items at crate root
pub use components::dock_area::{dock_area, DockAreaProps, TabContentRenderer};
pub use drop_zone::{DockSide, DropZone, HitZone};
pub use layout::{DockLayout, DockNode, FloatingPanel, NodeId, SplitDirection, Tab, TabContainer, TabId};
pub use state::{DockState, DragState};
pub use theme::DockTheme;
