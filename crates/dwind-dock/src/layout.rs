//! Core layout data structures for the dock system.
//!
//! The dock layout is represented as a tree of [`DockNode`]s, where:
//! - [`DockNode::Leaf`] contains a [`TabContainer`] with multiple tabs
//! - [`DockNode::Split`] divides space between two child nodes

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for a tab.
pub type TabId = String;

/// Unique identifier for a node in the layout tree.
pub type NodeId = u64;

/// Counter for generating unique node IDs.
static NODE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a new unique node ID.
pub fn generate_node_id() -> NodeId {
    NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Direction of a split divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Horizontal split: left | right
    Horizontal,
    /// Vertical split: top / bottom
    Vertical,
}

/// A single tab within a tab container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tab {
    /// Unique identifier for this tab.
    pub id: TabId,
    /// Display title shown in the tab bar.
    pub title: String,
    /// Whether the user can close this tab.
    pub closable: bool,
}

impl Tab {
    /// Create a new tab with the given ID and title.
    pub fn new(id: impl Into<TabId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            closable: true,
        }
    }

    /// Create a new tab that cannot be closed.
    pub fn new_permanent(id: impl Into<TabId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            closable: false,
        }
    }
}

/// A container holding multiple tabs with one active tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabContainer {
    /// The tabs in this container.
    pub tabs: Vec<Tab>,
    /// Index of the currently active tab.
    pub active_tab_index: usize,
}

impl TabContainer {
    /// Create a new tab container with the given tabs.
    /// The first tab will be active by default.
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tabs,
            active_tab_index: 0,
        }
    }

    /// Create a tab container with a single tab.
    pub fn single(tab: Tab) -> Self {
        Self::new(vec![tab])
    }

    /// Get a reference to the currently active tab, if any.
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    /// Get the ID of the currently active tab, if any.
    pub fn active_tab_id(&self) -> Option<&TabId> {
        self.active_tab().map(|t| &t.id)
    }

    /// Set the active tab by index, clamping to valid range.
    pub fn set_active_index(&mut self, index: usize) {
        if !self.tabs.is_empty() {
            self.active_tab_index = index.min(self.tabs.len() - 1);
        }
    }

    /// Find a tab by ID and return its index.
    pub fn find_tab_index(&self, tab_id: &TabId) -> Option<usize> {
        self.tabs.iter().position(|t| &t.id == tab_id)
    }

    /// Remove a tab by ID and return it if found.
    pub fn remove_tab(&mut self, tab_id: &TabId) -> Option<Tab> {
        if let Some(index) = self.find_tab_index(tab_id) {
            let tab = self.tabs.remove(index);
            // Adjust active index if needed
            if self.active_tab_index >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab_index = self.tabs.len() - 1;
            }
            Some(tab)
        } else {
            None
        }
    }

    /// Insert a tab at the given index.
    pub fn insert_tab(&mut self, index: usize, tab: Tab) {
        let index = index.min(self.tabs.len());
        self.tabs.insert(index, tab);
    }

    /// Check if this container is empty.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

/// A node in the dock layout tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DockNode {
    /// A leaf node containing tabs.
    Leaf {
        /// Unique identifier for this node.
        id: NodeId,
        /// The tab container.
        container: TabContainer,
    },
    /// A split node dividing space between two children.
    Split {
        /// Unique identifier for this node.
        id: NodeId,
        /// Direction of the split.
        direction: SplitDirection,
        /// Fraction (0.0 to 1.0) of space allocated to the first child.
        ratio: f64,
        /// First child (left or top).
        first: Box<DockNode>,
        /// Second child (right or bottom).
        second: Box<DockNode>,
    },
}

impl DockNode {
    /// Get the ID of this node.
    pub fn id(&self) -> NodeId {
        match self {
            DockNode::Leaf { id, .. } => *id,
            DockNode::Split { id, .. } => *id,
        }
    }

    /// Create a new leaf node with a single tab.
    pub fn leaf(tab: Tab) -> Self {
        DockNode::Leaf {
            id: generate_node_id(),
            container: TabContainer::single(tab),
        }
    }

    /// Create a new leaf node with multiple tabs.
    pub fn leaf_with_tabs(tabs: Vec<Tab>) -> Self {
        DockNode::Leaf {
            id: generate_node_id(),
            container: TabContainer::new(tabs),
        }
    }

    /// Create a horizontal split (left | right).
    pub fn hsplit(first: DockNode, second: DockNode, ratio: f64) -> Self {
        DockNode::Split {
            id: generate_node_id(),
            direction: SplitDirection::Horizontal,
            ratio: ratio.clamp(0.1, 0.9),
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Create a vertical split (top / bottom).
    pub fn vsplit(first: DockNode, second: DockNode, ratio: f64) -> Self {
        DockNode::Split {
            id: generate_node_id(),
            direction: SplitDirection::Vertical,
            ratio: ratio.clamp(0.1, 0.9),
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Check if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        matches!(self, DockNode::Leaf { .. })
    }

    /// Check if this is a split node.
    pub fn is_split(&self) -> bool {
        matches!(self, DockNode::Split { .. })
    }

    /// Get the tab container if this is a leaf node.
    pub fn container(&self) -> Option<&TabContainer> {
        match self {
            DockNode::Leaf { container, .. } => Some(container),
            DockNode::Split { .. } => None,
        }
    }

    /// Get a mutable reference to the tab container if this is a leaf node.
    pub fn container_mut(&mut self) -> Option<&mut TabContainer> {
        match self {
            DockNode::Leaf { container, .. } => Some(container),
            DockNode::Split { .. } => None,
        }
    }

    /// Find a node by ID in this subtree.
    pub fn find_node(&self, node_id: NodeId) -> Option<&DockNode> {
        if self.id() == node_id {
            return Some(self);
        }
        match self {
            DockNode::Leaf { .. } => None,
            DockNode::Split { first, second, .. } => {
                first.find_node(node_id).or_else(|| second.find_node(node_id))
            }
        }
    }

    /// Find a mutable reference to a node by ID in this subtree.
    pub fn find_node_mut(&mut self, node_id: NodeId) -> Option<&mut DockNode> {
        if self.id() == node_id {
            return Some(self);
        }
        match self {
            DockNode::Leaf { .. } => None,
            DockNode::Split { first, second, .. } => first
                .find_node_mut(node_id)
                .or_else(|| second.find_node_mut(node_id)),
        }
    }

    /// Check if this node or any descendant contains the given tab.
    pub fn contains_tab(&self, tab_id: &TabId) -> bool {
        match self {
            DockNode::Leaf { container, .. } => container.find_tab_index(tab_id).is_some(),
            DockNode::Split { first, second, .. } => {
                first.contains_tab(tab_id) || second.contains_tab(tab_id)
            }
        }
    }
}

/// A floating (detached) panel that hovers over the main dock area.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatingPanel {
    /// Unique identifier for this panel.
    pub id: NodeId,
    /// The tab container.
    pub container: TabContainer,
    /// X position in pixels.
    pub x: f64,
    /// Y position in pixels.
    pub y: f64,
    /// Width in pixels.
    pub width: f64,
    /// Height in pixels.
    pub height: f64,
}

impl FloatingPanel {
    /// Create a new floating panel with a single tab.
    pub fn new(tab: Tab, x: f64, y: f64) -> Self {
        Self {
            id: generate_node_id(),
            container: TabContainer::single(tab),
            x,
            y,
            width: 400.0,
            height: 300.0,
        }
    }

    /// Create a new floating panel with a tab container.
    pub fn with_container(container: TabContainer, x: f64, y: f64) -> Self {
        Self {
            id: generate_node_id(),
            container,
            x,
            y,
            width: 400.0,
            height: 300.0,
        }
    }
}

/// Current layout format version.
pub const LAYOUT_VERSION: u32 = 1;

/// The complete dock layout state.
///
/// This struct is serializable for layout persistence. The `version` field
/// enables future format migrations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DockLayout {
    /// Layout format version for future compatibility.
    ///
    /// Current version is 1. This field is used for migration when
    /// deserializing older layouts.
    #[serde(default = "default_version")]
    pub version: u32,
    /// The root of the docked layout tree. None if all panels are floating.
    pub root: Option<DockNode>,
    /// Floating panels not docked to the main tree.
    pub floating: Vec<FloatingPanel>,
}

fn default_version() -> u32 {
    LAYOUT_VERSION
}

impl DockLayout {
    /// Create a new layout with a root node.
    pub fn new(root: DockNode) -> Self {
        Self {
            version: LAYOUT_VERSION,
            root: Some(root),
            floating: Vec::new(),
        }
    }

    /// Create an empty layout.
    pub fn empty() -> Self {
        Self {
            version: LAYOUT_VERSION,
            ..Self::default()
        }
    }

    /// Find a node by ID in the docked tree or floating panels.
    pub fn find_node(&self, node_id: NodeId) -> Option<&DockNode> {
        self.root.as_ref().and_then(|r| r.find_node(node_id))
    }

    /// Check if the layout contains a tab with the given ID.
    pub fn contains_tab(&self, tab_id: &TabId) -> bool {
        let in_root = self.root.as_ref().map(|r| r.contains_tab(tab_id)).unwrap_or(false);
        let in_floating = self
            .floating
            .iter()
            .any(|p| p.container.find_tab_index(tab_id).is_some());
        in_root || in_floating
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_container_operations() {
        let mut container = TabContainer::new(vec![
            Tab::new("a", "Tab A"),
            Tab::new("b", "Tab B"),
            Tab::new("c", "Tab C"),
        ]);

        assert_eq!(container.active_tab_id(), Some(&"a".to_string()));

        container.set_active_index(1);
        assert_eq!(container.active_tab_id(), Some(&"b".to_string()));

        let removed = container.remove_tab(&"b".to_string());
        assert!(removed.is_some());
        assert_eq!(container.tabs.len(), 2);
        // Active index should adjust
        assert!(container.active_tab_index < container.tabs.len());
    }

    #[test]
    fn test_dock_node_find() {
        let leaf1 = DockNode::leaf(Tab::new("a", "A"));
        let leaf2 = DockNode::leaf(Tab::new("b", "B"));
        let leaf1_id = leaf1.id();
        let leaf2_id = leaf2.id();

        let split = DockNode::hsplit(leaf1, leaf2, 0.5);

        assert!(split.find_node(leaf1_id).is_some());
        assert!(split.find_node(leaf2_id).is_some());
        assert!(split.find_node(999).is_none());
    }

    #[test]
    fn test_layout_serialization() {
        let layout = DockLayout::new(DockNode::hsplit(
            DockNode::leaf(Tab::new("files", "Files")),
            DockNode::vsplit(
                DockNode::leaf_with_tabs(vec![
                    Tab::new("editor1", "main.rs"),
                    Tab::new("editor2", "lib.rs"),
                ]),
                DockNode::leaf(Tab::new("terminal", "Terminal")),
                0.7,
            ),
            0.25,
        ));

        let json = serde_json::to_string(&layout).unwrap();
        let restored: DockLayout = serde_json::from_str(&json).unwrap();

        assert_eq!(layout, restored);
    }
}
