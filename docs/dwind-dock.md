# dwind-dock - Dockable Panel System

Blender-style dockable panel system for DOMINATOR web applications. Supports tabs, splits, floating panels, and drag-and-drop.

## Setup

```rust
use dwind_dock::prelude::*;
use std::sync::Arc;
```

## Quick Start

```rust
// 1. Create layout
let layout = DockLayout::new(DockNode::hsplit(
    DockNode::leaf(Tab::new("files", "Files")),
    DockNode::leaf(Tab::new("editor", "Editor")),
    0.25,  // Left pane takes 25%
));

// 2. Create state with change callback
let state = DockState::new(layout, |layout| {
    // Persist layout changes here (e.g., localStorage)
});

// 3. Define content renderer
let tab_content: TabContentRenderer = Arc::new(|tab_id| {
    match tab_id.as_str() {
        "files" => html!("div", { .text("Files panel") }),
        "editor" => html!("div", { .text("Editor panel") }),
        _ => html!("div", { .text("Unknown") }),
    }
});

// 4. Render
dock_area(DockAreaProps {
    state,
    tab_content,
    theme: DockTheme::default(),
    apply: None,
})
```

## Data Structures

### DockLayout
```rust
pub struct DockLayout {
    pub root: Option<DockNode>,          // Main docked area
    pub floating: Vec<FloatingPanel>,    // Detached windows
}

DockLayout::new(root_node)    // Create with root
DockLayout::empty()           // Create empty
```

### DockNode
```rust
pub enum DockNode {
    Leaf { id: NodeId, container: TabContainer },
    Split { id: NodeId, direction: SplitDirection, ratio: f64, first: Box<DockNode>, second: Box<DockNode> },
}
```

**Building Nodes:**
```rust
// Single tab leaf
DockNode::leaf(Tab::new("id", "Title"))

// Multiple tabs
DockNode::leaf_with_tabs(vec![tab1, tab2, tab3])

// Horizontal split (left | right)
DockNode::hsplit(left, right, 0.3)

// Vertical split (top / bottom)
DockNode::vsplit(top, bottom, 0.7)
```

### Tab
```rust
pub struct Tab {
    pub id: TabId,        // String identifier
    pub title: String,    // Display title
    pub closable: bool,   // Can user close it?
}

Tab::new("editor", "Editor")           // Closable tab
Tab::new_permanent("files", "Files")   // Not closable
```

### FloatingPanel
```rust
pub struct FloatingPanel {
    pub id: NodeId,
    pub container: TabContainer,
    pub x: f64, pub y: f64,           // Position in pixels
    pub width: f64, pub height: f64,  // Size in pixels
}
```

## Layout Examples

### IDE-Style Layout
```rust
DockLayout::new(DockNode::vsplit(
    // Top: sidebar + editor
    DockNode::hsplit(
        DockNode::leaf_with_tabs(vec![
            Tab::new("files", "Files"),
            Tab::new("search", "Search"),
        ]),
        DockNode::leaf_with_tabs(vec![
            Tab::new("editor", "Editor"),
            Tab::new("preview", "Preview"),
        ]),
        0.25,
    ),
    // Bottom: console
    DockNode::leaf_with_tabs(vec![
        Tab::new("console", "Console"),
        Tab::new("output", "Output"),
    ]),
    0.75,
))
```

### Three-Column Layout
```rust
DockLayout::new(DockNode::hsplit(
    DockNode::leaf(Tab::new("left", "Left")),
    DockNode::hsplit(
        DockNode::leaf(Tab::new("center", "Center")),
        DockNode::leaf(Tab::new("right", "Right")),
        0.7,
    ),
    0.2,
))
```

## DockState

Central state manager (cheaply cloneable via Arc):

```rust
// Create with change callback
let state = DockState::new(layout, |layout| {
    let json = serde_json::to_string(layout).unwrap();
    // Save to localStorage...
});

// Or without callback
let state = DockState::new_without_callback(layout);
```

### State Operations

```rust
// Tab operations
state.set_active_tab(node_id, tab_index);
state.close_tab(node_id, &tab_id);

// Split operations
state.set_split_ratio(node_id, 0.4);  // Clamped to 0.1-0.9

// Floating panel operations
state.move_floating_panel(panel_id, x, y);
state.resize_floating_panel(panel_id, width, height);

// Layout operations
state.set_layout(new_layout);
let current = state.layout();  // Get snapshot
```

### Reactive Signals

```rust
state.layout_signal()       // Signal<Item = DockLayout>
state.drag_signal()         // Signal<Item = Option<DragState>>
state.is_dragging_signal()  // Signal<Item = bool>
```

## Drop Zones

When dragging tabs, these drop zones are available:

```rust
pub enum DropZone {
    TabBar { node_id, insert_index },   // Add to tab bar
    Split { node_id, side },            // Split and dock
    Float { x, y },                     // Create floating panel
    RootEdge { side },                  // Dock to root edge
}

pub enum DockSide {
    Top,     // Split vertically, new on top
    Bottom,  // Split vertically, new on bottom
    Left,    // Split horizontally, new on left
    Right,   // Split horizontally, new on right
    Center,  // Add as tab (TabBar only)
}
```

## Theme

```rust
pub struct DockTheme {
    pub tab_bar_height: f64,        // rem (default: 2.0)
    pub divider_thickness: f64,     // px (default: 4.0)
    pub border_radius: String,      // CSS (default: "0")
    pub animation_duration_ms: u32, // ms (default: 150)
    pub min_panel_size: f64,        // px (default: 100.0)
}

// Presets
DockTheme::default()
DockTheme::rounded()   // With 0.25rem radius
DockTheme::compact()   // Smaller tabs (1.5rem)

// Builder
DockTheme::default()
    .with_tab_bar_height(1.5)
    .with_divider_thickness(2.0)
    .with_border_radius("0.5rem")
```

## Serialization

```rust
// Save
let json = serde_json::to_string(&state.layout())?;

// Restore
let layout: DockLayout = serde_json::from_str(&json)?;
state.set_layout(layout);
```

## Layout Queries

```rust
let layout = state.layout();

// Find node by ID
if let Some(node) = layout.find_node(node_id) { ... }

// Check if tab exists
if layout.contains_tab(&tab_id) { ... }
```

## Components

| Component | Description |
|-----------|-------------|
| `dock_area` | Root container, renders entire dock system |
| `tab_panel` | Leaf node with tab bar and content |
| `tab_bar` | Tab headers with click/close handling |
| `split_container` | Two children with resizable divider |
| `floating_panel` | Draggable/resizable overlay panel |
| `drag_overlay` | Visual feedback during drag |
| `drop_zone_overlay` | Drop target indicators |

## Full Example

```rust
use dwind_dock::prelude::*;
use dominator::{html, Dom};
use std::sync::Arc;

fn create_dock() -> Dom {
    let layout = DockLayout::new(DockNode::vsplit(
        DockNode::hsplit(
            DockNode::leaf_with_tabs(vec![
                Tab::new("files", "Files"),
                Tab::new_permanent("props", "Properties"),
            ]),
            DockNode::leaf(Tab::new("editor", "Editor")),
            0.3,
        ),
        DockNode::leaf_with_tabs(vec![
            Tab::new("console", "Console"),
            Tab::new("output", "Output"),
        ]),
        0.7,
    ));

    let state = DockState::new(layout, |_| {});

    let tab_content: TabContentRenderer = Arc::new(|tab_id| {
        html!("div", {
            .dwclass!("p-4 h-full")
            .text(&format!("Content for: {}", tab_id))
        })
    });

    html!("div", {
        .dwclass!("w-full h-screen")
        .child(dock_area(DockAreaProps {
            state,
            tab_content,
            theme: DockTheme::default(),
            apply: None,
        }))
    })
}
```
