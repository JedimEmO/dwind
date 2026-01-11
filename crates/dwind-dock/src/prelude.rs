//! Convenient re-exports for common usage.
//!
//! ```ignore
//! use dwind_dock::prelude::*;
//! ```

// Layout types
pub use crate::layout::{
    generate_node_id, DockLayout, DockNode, FloatingPanel, NodeId, SplitDirection, Tab,
    TabContainer, TabId, LAYOUT_VERSION,
};

// Drop zone types
pub use crate::drop_zone::{DockSide, DropZone, HitZone};

// State management
pub use crate::state::{DockState, DragState, OnLayoutChange};

// Theme
pub use crate::theme::DockTheme;

// Components
pub use crate::components::prelude::*;
