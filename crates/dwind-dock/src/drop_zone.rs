//! Drop zone types for drag-and-drop docking.

use crate::layout::NodeId;
use serde::{Deserialize, Serialize};

/// The side of a panel where content can be docked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DockSide {
    /// Top edge (creates vertical split, new panel on top).
    Top,
    /// Bottom edge (creates vertical split, new panel on bottom).
    Bottom,
    /// Left edge (creates horizontal split, new panel on left).
    Left,
    /// Right edge (creates horizontal split, new panel on right).
    Right,
    /// Center (adds tab to existing tab bar).
    Center,
}

impl DockSide {
    /// Check if this side creates a horizontal split.
    pub fn is_horizontal(&self) -> bool {
        matches!(self, DockSide::Left | DockSide::Right)
    }

    /// Check if this side creates a vertical split.
    pub fn is_vertical(&self) -> bool {
        matches!(self, DockSide::Top | DockSide::Bottom)
    }

    /// Check if the new panel should be the first child in the split.
    pub fn is_first(&self) -> bool {
        matches!(self, DockSide::Top | DockSide::Left)
    }
}

/// Represents where a dragged tab can be dropped.
#[derive(Debug, Clone, PartialEq)]
pub enum DropZone {
    /// Drop as a new tab in an existing tab bar.
    TabBar {
        /// The node containing the tab bar.
        node_id: NodeId,
        /// Index where to insert the tab (usize::MAX for end).
        insert_index: usize,
    },
    /// Split a node and dock to one side.
    Split {
        /// The node to split.
        node_id: NodeId,
        /// Which side to dock to.
        side: DockSide,
    },
    /// Create a new floating panel at the given position.
    Float {
        /// X position for the new floating panel.
        x: f64,
        /// Y position for the new floating panel.
        y: f64,
    },
    /// Dock to an edge of the entire dock area.
    RootEdge {
        /// Which edge to dock to.
        side: DockSide,
    },
}

impl DropZone {
    /// Create a drop zone for adding to a tab bar.
    pub fn tab_bar(node_id: NodeId) -> Self {
        DropZone::TabBar {
            node_id,
            insert_index: usize::MAX,
        }
    }

    /// Create a drop zone for adding to a tab bar at a specific index.
    pub fn tab_bar_at(node_id: NodeId, index: usize) -> Self {
        DropZone::TabBar {
            node_id,
            insert_index: index,
        }
    }

    /// Create a drop zone for splitting a node.
    pub fn split(node_id: NodeId, side: DockSide) -> Self {
        DropZone::Split { node_id, side }
    }

    /// Create a drop zone for a floating panel.
    pub fn floating(x: f64, y: f64) -> Self {
        DropZone::Float { x, y }
    }

    /// Create a drop zone for the root edge.
    pub fn root_edge(side: DockSide) -> Self {
        DropZone::RootEdge { side }
    }
}

/// Hit-test result when determining which drop zone the mouse is over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitZone {
    /// Over the center (tab bar area).
    Center,
    /// Over the top edge.
    Top,
    /// Over the bottom edge.
    Bottom,
    /// Over the left edge.
    Left,
    /// Over the right edge.
    Right,
    /// Not over any zone.
    None,
}

impl HitZone {
    /// Determine which zone a point is in, given the bounds of an element.
    ///
    /// The edge zones are the outer 25% on each side, with the center being
    /// the inner 50%.
    pub fn from_point(x: f64, y: f64, width: f64, height: f64) -> Self {
        let edge_ratio = 0.25;

        let left_edge = width * edge_ratio;
        let right_edge = width * (1.0 - edge_ratio);
        let top_edge = height * edge_ratio;
        let bottom_edge = height * (1.0 - edge_ratio);

        // Check edges first (they take priority)
        if x < left_edge {
            HitZone::Left
        } else if x > right_edge {
            HitZone::Right
        } else if y < top_edge {
            HitZone::Top
        } else if y > bottom_edge {
            HitZone::Bottom
        } else {
            HitZone::Center
        }
    }

    /// Convert to a DockSide, if applicable.
    pub fn to_dock_side(self) -> Option<DockSide> {
        match self {
            HitZone::Center => Some(DockSide::Center),
            HitZone::Top => Some(DockSide::Top),
            HitZone::Bottom => Some(DockSide::Bottom),
            HitZone::Left => Some(DockSide::Left),
            HitZone::Right => Some(DockSide::Right),
            HitZone::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_zone_from_point() {
        // 100x100 box
        let width = 100.0;
        let height = 100.0;

        // Center
        assert_eq!(HitZone::from_point(50.0, 50.0, width, height), HitZone::Center);

        // Edges
        assert_eq!(HitZone::from_point(10.0, 50.0, width, height), HitZone::Left);
        assert_eq!(HitZone::from_point(90.0, 50.0, width, height), HitZone::Right);
        assert_eq!(HitZone::from_point(50.0, 10.0, width, height), HitZone::Top);
        assert_eq!(HitZone::from_point(50.0, 90.0, width, height), HitZone::Bottom);
    }
}
