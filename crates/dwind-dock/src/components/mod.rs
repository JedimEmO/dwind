//! UI components for the dock system.

pub mod dock_area;
pub mod drag_overlay;
pub mod drop_zone_overlay;
pub mod floating_panel;
pub mod split_container;
pub mod split_divider;
pub mod tab_bar;
pub mod tab_panel;

/// Prelude for components.
pub mod prelude {
    pub use super::dock_area::{dock_area, DockAreaProps, TabContentRenderer};
    pub use super::drag_overlay::{drag_overlay, DragOverlayProps};
    pub use super::drop_zone_overlay::{drop_zone_overlay, DropZoneOverlayProps};
    pub use super::floating_panel::{floating_panel, FloatingPanelProps};
    pub use super::split_container::{split_container, SplitContainerProps};
    pub use super::split_divider::{split_divider, SplitDividerProps};
    pub use super::tab_bar::{tab_bar, tab_header, TabBarProps, TabHeaderProps};
    pub use super::tab_panel::{tab_panel, TabPanelProps};
}
