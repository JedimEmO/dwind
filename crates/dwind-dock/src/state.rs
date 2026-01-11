//! State management for the dock system.
//!
//! The [`DockState`] is the central state manager that coordinates layout changes,
//! drag-and-drop operations, and notifies listeners of updates.

use crate::drop_zone::{DockSide, DropZone};
use crate::layout::{
    generate_node_id, DockLayout, DockNode, FloatingPanel, NodeId, SplitDirection, Tab,
    TabContainer, TabId,
};
use futures_signals::signal::{Mutable, Signal};
use std::cell::RefCell;
use std::sync::Arc;

/// State for an active drag operation.
#[derive(Debug, Clone)]
pub struct DragState {
    /// The tab being dragged.
    pub tab: Tab,
    /// Source node where the drag started.
    pub source_node_id: NodeId,
    /// Current mouse X position.
    pub mouse_x: f64,
    /// Current mouse Y position.
    pub mouse_y: f64,
}

/// Callback type for layout changes.
pub type OnLayoutChange = Arc<dyn Fn(&DockLayout) + Send + Sync>;

/// Central state manager for the dock system.
///
/// `DockState` manages the layout tree, drag operations, and notifies listeners
/// of layout changes. It is the primary interface for interacting with the dock
/// system programmatically.
///
/// # Cloning
///
/// `DockState` is cheaply cloneable because it uses `Arc` internally. You don't
/// need to wrap it in `Arc` yourself - just clone it directly when passing to
/// multiple components or callbacks.
///
/// # Example
///
/// ```ignore
/// let state = DockState::new(layout, |layout| {
///     // Persist layout changes
/// });
///
/// // Clone freely - this is cheap
/// let state2 = state.clone();
/// ```
///
/// # Architecture Note
///
/// The `pending_drop_zone` field uses `RefCell` instead of `Mutable` to avoid
/// triggering signal updates on every mouse hover during drag operations. This
/// is a deliberate trade-off: hover state changes frequently but doesn't need
/// to trigger re-renders, while the actual drop operation reads this value
/// synchronously when the mouse is released.
#[derive(Clone)]
pub struct DockState {
    /// The layout tree.
    layout: Arc<Mutable<DockLayout>>,
    /// Current drag operation (if any).
    drag_state: Arc<Mutable<Option<DragState>>>,
    /// Pending drop zone (set by drop zone on hover, read on mouseup).
    ///
    /// Uses RefCell instead of Mutable to avoid signal updates on hover.
    /// This is intentional - see struct-level docs for rationale.
    pending_drop_zone: Arc<RefCell<Option<DropZone>>>,
    /// Callback when layout changes.
    on_layout_change: OnLayoutChange,
}

impl DockState {
    /// Create a new dock state with an initial layout.
    ///
    /// The `on_layout_change` callback is invoked whenever the layout changes,
    /// allowing you to persist the layout (e.g., to localStorage).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = DockState::new(layout, |layout| {
    ///     let json = serde_json::to_string(layout).unwrap();
    ///     // Save to localStorage
    /// });
    /// ```
    pub fn new(
        initial_layout: DockLayout,
        on_layout_change: impl Fn(&DockLayout) + Send + Sync + 'static,
    ) -> Self {
        Self {
            layout: Arc::new(Mutable::new(initial_layout)),
            drag_state: Arc::new(Mutable::new(None)),
            pending_drop_zone: Arc::new(RefCell::new(None)),
            on_layout_change: Arc::new(on_layout_change),
        }
    }

    /// Create a new dock state without a change callback.
    ///
    /// Useful for testing or when you don't need to persist layout changes.
    pub fn new_without_callback(initial_layout: DockLayout) -> Self {
        Self::new(initial_layout, |_| {})
    }

    // --- Signals ---

    /// Get a signal for the entire layout.
    ///
    /// This signal emits the full `DockLayout` whenever any part of it changes.
    /// Use this for reactive rendering of the dock tree.
    ///
    /// Note: This triggers a full re-render on any change. For large layouts,
    /// consider whether you need the full layout or just specific parts.
    pub fn layout_signal(&self) -> impl Signal<Item = DockLayout> {
        self.layout.signal_cloned()
    }

    /// Get a signal for the current drag state.
    ///
    /// Emits `Some(DragState)` during a drag operation, `None` otherwise.
    /// Useful for rendering drag overlays and visual feedback.
    pub fn drag_signal(&self) -> impl Signal<Item = Option<DragState>> {
        self.drag_state.signal_cloned()
    }

    /// Get a signal indicating if a drag is in progress.
    ///
    /// More efficient than `drag_signal()` when you only need to know
    /// whether dragging is happening, not the drag details.
    pub fn is_dragging_signal(&self) -> impl Signal<Item = bool> {
        self.drag_state.signal_ref(|s| s.is_some())
    }

    /// Get the current layout as a snapshot.
    ///
    /// This clones the current layout state. For reactive updates,
    /// use `layout_signal()` instead.
    pub fn layout(&self) -> DockLayout {
        self.layout.get_cloned()
    }

    /// Replace the entire layout.
    ///
    /// Use this for workspace switching or restoring a saved layout.
    /// Triggers the `on_layout_change` callback.
    pub fn set_layout(&self, new_layout: DockLayout) {
        self.layout.set(new_layout.clone());
        (self.on_layout_change)(&new_layout);
    }

    // --- Drag Operations ---

    /// Start dragging a tab.
    ///
    /// Called when a tab drag begins. The `x` and `y` are the initial
    /// mouse coordinates for positioning the drag overlay.
    pub fn start_drag(&self, tab: Tab, source_node_id: NodeId, x: f64, y: f64) {
        self.drag_state.set(Some(DragState {
            tab,
            source_node_id,
            mouse_x: x,
            mouse_y: y,
        }));
    }

    /// Update the drag position during a drag operation.
    ///
    /// Called on mouse move to update the drag overlay position.
    /// Does nothing if no drag is in progress.
    pub fn update_drag(&self, x: f64, y: f64) {
        if let Some(state) = self.drag_state.lock_mut().as_mut() {
            state.mouse_x = x;
            state.mouse_y = y;
        }
    }

    /// Set the pending drop zone.
    ///
    /// Called by drop zone indicators on mouse enter/leave to register
    /// where a drop would occur. This uses `RefCell` internally to avoid
    /// signal updates on every hover.
    pub fn set_pending_drop_zone(&self, zone: Option<DropZone>) {
        *self.pending_drop_zone.borrow_mut() = zone;
    }

    /// Complete the drag operation.
    ///
    /// If a pending drop zone is set, the tab is moved to that zone.
    /// Otherwise, the drag is cancelled with no effect.
    pub fn end_drag(&self) {
        let drag = self.drag_state.replace(None);
        let pending_zone = self.pending_drop_zone.borrow_mut().take();

        if let Some(drag_state) = drag {
            if let Some(zone) = pending_zone {
                self.apply_drop(drag_state.tab, drag_state.source_node_id, zone);
            }
        }
    }

    /// Cancel the drag operation without applying any changes.
    ///
    /// Clears both the drag state and any pending drop zone to prevent
    /// stale drop zones from being applied on subsequent drags.
    pub fn cancel_drag(&self) {
        self.drag_state.set(None);
        self.pending_drop_zone.borrow_mut().take();
    }

    /// Check if a drag is currently in progress.
    ///
    /// For reactive UI updates, prefer `is_dragging_signal()`.
    pub fn is_dragging(&self) -> bool {
        self.drag_state.lock_ref().is_some()
    }

    // --- Layout Operations ---

    /// Set the active tab for a node by index.
    ///
    /// The index is clamped to valid range. Triggers `on_layout_change`.
    pub fn set_active_tab(&self, node_id: NodeId, tab_index: usize) {
        let mut layout = self.layout.lock_mut();
        if let Some(root) = &mut layout.root {
            if let Some(node) = root.find_node_mut(node_id) {
                if let Some(container) = node.container_mut() {
                    container.set_active_index(tab_index);
                }
            }
        }
        // Also check floating panels
        for panel in &mut layout.floating {
            if panel.id == node_id {
                panel.container.set_active_index(tab_index);
            }
        }
        (self.on_layout_change)(&layout);
    }

    /// Update the split ratio for a split node.
    ///
    /// The ratio is clamped to the range 0.1 to 0.9 to ensure both
    /// children remain visible. Triggers `on_layout_change`.
    pub fn set_split_ratio(&self, node_id: NodeId, ratio: f64) {
        let mut layout = self.layout.lock_mut();
        let ratio = ratio.clamp(0.1, 0.9);
        if let Some(root) = &mut layout.root {
            Self::update_split_ratio_recursive(root, node_id, ratio);
        }
        (self.on_layout_change)(&layout);
    }

    fn update_split_ratio_recursive(node: &mut DockNode, node_id: NodeId, ratio: f64) -> bool {
        match node {
            DockNode::Leaf { .. } => false,
            DockNode::Split {
                id,
                ratio: node_ratio,
                first,
                second,
                ..
            } => {
                if *id == node_id {
                    *node_ratio = ratio;
                    true
                } else {
                    Self::update_split_ratio_recursive(first, node_id, ratio)
                        || Self::update_split_ratio_recursive(second, node_id, ratio)
                }
            }
        }
    }

    /// Close a tab by ID.
    ///
    /// Removes the tab from the specified node. If the node becomes empty,
    /// it is automatically removed and its parent split is cleaned up.
    /// Triggers `on_layout_change`.
    pub fn close_tab(&self, node_id: NodeId, tab_id: &TabId) {
        let mut layout = self.layout.lock_mut();
        Self::remove_tab_from_layout(&mut layout, node_id, tab_id);
        Self::cleanup_empty_nodes(&mut layout);
        (self.on_layout_change)(&layout);
    }

    /// Move a floating panel to new coordinates.
    ///
    /// The coordinates are in pixels relative to the dock area.
    /// Triggers `on_layout_change`.
    pub fn move_floating_panel(&self, panel_id: NodeId, x: f64, y: f64) {
        let mut layout = self.layout.lock_mut();
        if let Some(panel) = layout.floating.iter_mut().find(|p| p.id == panel_id) {
            panel.x = x;
            panel.y = y;
        }
        (self.on_layout_change)(&layout);
    }

    /// Resize a floating panel.
    ///
    /// Dimensions are clamped to minimum values (200x150 pixels).
    /// Triggers `on_layout_change`.
    pub fn resize_floating_panel(&self, panel_id: NodeId, width: f64, height: f64) {
        let mut layout = self.layout.lock_mut();
        if let Some(panel) = layout.floating.iter_mut().find(|p| p.id == panel_id) {
            panel.width = width.max(200.0);
            panel.height = height.max(150.0);
        }
        (self.on_layout_change)(&layout);
    }

    // --- Drop Handling ---

    fn apply_drop(&self, tab: Tab, source_node_id: NodeId, zone: DropZone) {
        let mut layout = self.layout.lock_mut();

        // Remove tab from source
        Self::remove_tab_from_layout(&mut layout, source_node_id, &tab.id);

        // Add to destination based on zone
        match zone {
            DropZone::TabBar {
                node_id,
                insert_index,
            } => {
                Self::insert_tab_at(&mut layout, node_id, tab, insert_index);
            }
            DropZone::Split { node_id, side } => {
                if side == DockSide::Center {
                    // Treat center as tab bar
                    Self::insert_tab_at(&mut layout, node_id, tab, usize::MAX);
                } else {
                    Self::split_and_insert(&mut layout, node_id, tab, side);
                }
            }
            DropZone::Float { x, y } => {
                layout.floating.push(FloatingPanel::new(tab, x, y));
            }
            DropZone::RootEdge { side } => {
                Self::dock_to_root_edge(&mut layout, tab, side);
            }
        }

        // Clean up empty nodes
        Self::cleanup_empty_nodes(&mut layout);

        // Notify listener
        (self.on_layout_change)(&layout);
    }

    fn remove_tab_from_layout(layout: &mut DockLayout, node_id: NodeId, tab_id: &TabId) {
        // Try to remove from docked nodes
        if let Some(root) = &mut layout.root {
            Self::remove_tab_recursive(root, node_id, tab_id);
        }

        // Try to remove from floating panels
        for panel in &mut layout.floating {
            if panel.id == node_id {
                panel.container.remove_tab(tab_id);
            }
        }
    }

    fn remove_tab_recursive(node: &mut DockNode, node_id: NodeId, tab_id: &TabId) -> bool {
        match node {
            DockNode::Leaf { id, container, .. } => {
                if *id == node_id {
                    container.remove_tab(tab_id);
                    true
                } else {
                    false
                }
            }
            DockNode::Split { first, second, .. } => {
                Self::remove_tab_recursive(first, node_id, tab_id)
                    || Self::remove_tab_recursive(second, node_id, tab_id)
            }
        }
    }

    fn insert_tab_at(layout: &mut DockLayout, node_id: NodeId, tab: Tab, index: usize) {
        // Try to insert in docked nodes
        if let Some(root) = &mut layout.root {
            if Self::insert_tab_recursive(root, node_id, tab.clone(), index) {
                return;
            }
        }

        // Try to insert in floating panels
        for panel in &mut layout.floating {
            if panel.id == node_id {
                let idx = index.min(panel.container.tabs.len());
                panel.container.insert_tab(idx, tab);
                panel.container.set_active_index(idx);
                return;
            }
        }
    }

    fn insert_tab_recursive(
        node: &mut DockNode,
        node_id: NodeId,
        tab: Tab,
        index: usize,
    ) -> bool {
        match node {
            DockNode::Leaf { id, container, .. } => {
                if *id == node_id {
                    let idx = index.min(container.tabs.len());
                    container.insert_tab(idx, tab);
                    container.set_active_index(idx);
                    true
                } else {
                    false
                }
            }
            DockNode::Split { first, second, .. } => {
                Self::insert_tab_recursive(first, node_id, tab.clone(), index)
                    || Self::insert_tab_recursive(second, node_id, tab, index)
            }
        }
    }

    fn split_and_insert(layout: &mut DockLayout, node_id: NodeId, tab: Tab, side: DockSide) {
        if let Some(root) = &mut layout.root {
            Self::split_and_insert_recursive(root, node_id, tab, side);
        }
    }

    fn split_and_insert_recursive(
        node: &mut DockNode,
        node_id: NodeId,
        tab: Tab,
        side: DockSide,
    ) -> bool {
        if node.id() == node_id {
            // Found the target node - wrap it in a split
            let old_node = std::mem::replace(
                node,
                DockNode::Leaf {
                    id: 0,
                    container: TabContainer::new(vec![]),
                },
            );

            let new_leaf = DockNode::leaf(tab);
            let direction = if side.is_horizontal() {
                SplitDirection::Horizontal
            } else {
                SplitDirection::Vertical
            };

            let (first, second) = if side.is_first() {
                (new_leaf, old_node)
            } else {
                (old_node, new_leaf)
            };

            *node = DockNode::Split {
                id: generate_node_id(),
                direction,
                ratio: 0.5,
                first: Box::new(first),
                second: Box::new(second),
            };

            return true;
        }

        match node {
            DockNode::Leaf { .. } => false,
            DockNode::Split { first, second, .. } => {
                Self::split_and_insert_recursive(first, node_id, tab.clone(), side)
                    || Self::split_and_insert_recursive(second, node_id, tab, side)
            }
        }
    }

    fn dock_to_root_edge(layout: &mut DockLayout, tab: Tab, side: DockSide) {
        let new_leaf = DockNode::leaf(tab);

        let direction = if side.is_horizontal() {
            SplitDirection::Horizontal
        } else {
            SplitDirection::Vertical
        };

        if let Some(root) = layout.root.take() {
            let (first, second) = if side.is_first() {
                (new_leaf, root)
            } else {
                (root, new_leaf)
            };

            layout.root = Some(DockNode::Split {
                id: generate_node_id(),
                direction,
                ratio: if side.is_first() { 0.25 } else { 0.75 },
                first: Box::new(first),
                second: Box::new(second),
            });
        } else {
            layout.root = Some(new_leaf);
        }
    }

    fn cleanup_empty_nodes(layout: &mut DockLayout) {
        // Remove empty floating panels
        layout.floating.retain(|p| !p.container.is_empty());

        // Clean up docked tree
        if let Some(root) = &mut layout.root {
            layout.root = Self::cleanup_node(std::mem::replace(
                root,
                DockNode::Leaf {
                    id: 0,
                    container: TabContainer::new(vec![]),
                },
            ));
        }
    }

    fn cleanup_node(node: DockNode) -> Option<DockNode> {
        match node {
            DockNode::Leaf { container, .. } => {
                if container.is_empty() {
                    None
                } else {
                    Some(DockNode::Leaf {
                        id: generate_node_id(),
                        container,
                    })
                }
            }
            DockNode::Split {
                direction,
                ratio,
                first,
                second,
                ..
            } => {
                let first = Self::cleanup_node(*first);
                let second = Self::cleanup_node(*second);

                match (first, second) {
                    (Some(f), Some(s)) => Some(DockNode::Split {
                        id: generate_node_id(),
                        direction,
                        ratio,
                        first: Box::new(f),
                        second: Box::new(s),
                    }),
                    (Some(node), None) | (None, Some(node)) => Some(node),
                    (None, None) => None,
                }
            }
        }
    }
}
