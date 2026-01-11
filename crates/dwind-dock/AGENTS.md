# AGENTS.md

This file provides guidance for AI agents working with the `dwind-dock` crate.

## Overview

**dwind-dock** is a Blender-style dockable panel system for DOMINATOR web applications. It provides:

- Draggable, resizable split panels
- Tabbed containers with closable tabs
- Drag-and-drop tab reordering between panels
- Floating (detached) panels
- Serializable layouts for persistence
- Reactive state management with `futures-signals`

## Architecture

### Layout Tree

The dock layout is a tree structure defined in `src/layout.rs`:

```
DockLayout
├── root: Option<DockNode>     // Main docked area
└── floating: Vec<FloatingPanel>  // Detached windows

DockNode (recursive)
├── Leaf { id, container: TabContainer }
└── Split { id, direction, ratio, first, second }

TabContainer
├── tabs: Vec<Tab>
└── active_tab_index: usize
```

### State Management

`DockState` (`src/state.rs`) is the central state manager:

- Wraps layout in `Mutable<DockLayout>` for reactivity
- Provides signals for UI updates (`layout_signal()`, `drag_signal()`)
- Handles all mutations (close tab, split, move, resize)
- Notifies listeners via callback on layout changes

### Component Hierarchy

```
dock_area (root container)
├── render_dock_node (recursive)
│   ├── tab_panel (leaf nodes)
│   │   ├── tab_bar
│   │   │   └── tab_header (per tab)
│   │   ├── tab content area
│   │   └── drop_zone_overlay (during drag)
│   └── split_container (split nodes)
│       ├── first child
│       ├── resize_handle
│       └── second child
├── floating_panel (for each floating)
└── drag_overlay (during drag operations)
```

## Key Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | Public API, prelude exports |
| `src/layout.rs` | Data structures: `DockNode`, `Tab`, `TabContainer`, `DockLayout` |
| `src/state.rs` | `DockState` - reactive state management |
| `src/theme.rs` | `DockTheme` - styling configuration |
| `src/drop_zone.rs` | `DropZone`, `DockSide` - drop target definitions |
| `src/components/dock_area.rs` | Root component, renders entire dock system |
| `src/components/tab_bar.rs` | Tab headers with close buttons |
| `src/components/tab_panel.rs` | Leaf node: tab bar + content |
| `src/components/split_container.rs` | Split node with draggable divider |
| `src/components/floating_panel.rs` | Detached window panels |
| `src/components/drag_overlay.rs` | Visual feedback during drag |
| `src/components/drop_zone_overlay.rs` | Drop target indicators |
| `src/components/resize_handle.rs` | Draggable split divider |

## Important Patterns

### DockState is Cheaply Cloneable

`DockState` uses `Arc` internally for all its fields. This means you can clone it freely without wrapping it in `Arc` yourself:

```rust
// Good - just clone directly
let state = DockState::new(layout, |_| {});
let state2 = state.clone(); // Cheap, just Arc::clone internally

// Unnecessary - don't wrap in Arc
let state = Arc::new(DockState::new(layout, |_| {})); // Not needed!
```

### RefCell for Hover State

The `DockState` uses `RefCell<Option<DropZone>>` for `pending_drop_zone` instead of `Mutable`. This is a deliberate architectural choice:

**Why RefCell instead of Mutable?**
- During drag operations, the mouse hovers over many drop zones rapidly
- Using `Mutable` would trigger signal updates and potentially re-renders on every hover
- `RefCell` allows synchronous read/write without signal updates
- The drop zone is only read once when the mouse is released (`end_drag()`)

**Trade-off:** This mixes signal-based reactivity with interior mutability, which can be confusing. However, the performance benefit (no signal updates during hover) outweighs the architectural impurity.

### Reactive Re-rendering

The `dock_area` component uses `child_signal()` with `layout_signal()`:

```rust
.child_signal(state.layout_signal().map(|layout| {
    layout.root.map(|root| render_dock_node(&root, ...))
}))
```

When state changes, the entire layout tree re-renders. This is intentional for simplicity but has implications:

### Event Handling Gotcha

**Close buttons must use `mousedown`, not `click`.**

The parent tab header has mousedown handlers for drag detection. When mousedown fires, it can trigger state updates that cause re-renders. If the DOM element is replaced before `click` fires, the click handler never executes.

```rust
// CORRECT: Handle on mousedown
.event(move |e: events::MouseDown| {
    e.stop_propagation();
    state.close_tab(node_id, &tab.id);
})

// WRONG: Click may never fire after re-render
.event(move |e: events::Click| {
    state.close_tab(node_id, &tab.id);
})
```

### Drop Zone Event Ordering

**Drop zones must set state on `MouseEnter`, not `MouseUp`.**

The `dock_area` has a global MouseUp handler that calls `end_drag()`. If drop zones try to set `hovered_zone` on their own MouseUp event, it fires **after** the global handler has already processed the drop with `hovered_zone = None`.

```rust
// CORRECT: Set zone when mouse enters the drop indicator
.event(move |_: events::MouseEnter| {
    state.set_hovered_zone(Some(zone));
})
.event(move |_: events::MouseLeave| {
    state.set_hovered_zone(None);
})

// WRONG: MouseUp fires after global handler - too late!
.event(move |_: events::MouseUp| {
    state.set_hovered_zone(Some(zone));
})
```

### Node IDs

Each `DockNode` has a unique `NodeId` (u64) generated via `generate_node_id()`. These IDs:
- Identify nodes for operations (close, split, move)
- Are NOT stable across serialization/deserialization
- Are regenerated when layouts are created

### Tab IDs

`TabId` is a `String` that uniquely identifies tab content. The `TabContentRenderer` function receives this ID to render appropriate content:

```rust
let tab_content: TabContentRenderer = Arc::new(|tab_id| {
    match tab_id.as_str() {
        "files" => render_files_panel(),
        "editor" => render_editor_panel(),
        _ => html!("div", { .text("Unknown tab") }),
    }
});
```

## Usage Example

```rust
use dwind_dock::prelude::*;

// Define layout
let layout = DockLayout::new(DockNode::hsplit(
    DockNode::leaf(Tab::new("files", "Files")),
    DockNode::vsplit(
        DockNode::leaf_with_tabs(vec![
            Tab::new("editor", "Editor"),
            Tab::new("preview", "Preview"),
        ]),
        DockNode::leaf(Tab::new("console", "Console")),
        0.7,
    ),
    0.25,
));

// Create state
let state = DockState::new(layout, |layout| {
    // Persist layout changes here (e.g., localStorage)
});

// Render
dock_area(DockAreaProps {
    state,
    tab_content: Arc::new(|tab_id| render_content(tab_id)),
    theme: DockTheme::default(),
    apply: None,
})
```

## Testing

```bash
# Run unit tests
cargo test -p dwind-dock

# Test in browser (requires example app)
cd examples/webpage
trunk serve --open
# Navigate to #/dock-demo
```

## Serialization

Layouts can be serialized/deserialized with serde:

```rust
let json = serde_json::to_string(&layout)?;
let restored: DockLayout = serde_json::from_str(&json)?;
state.set_layout(restored);
```

Note: Node IDs are serialized but may conflict with runtime-generated IDs. For persistence, consider resetting IDs after deserialization.

## Styling

**Note:** Theme support is not yet fully implemented. The `DockTheme` struct is accepted by `DockAreaProps` for API stability, but currently does not affect rendering. All styling is hardcoded using dwind utility classes (gray-800, gray-900, etc.).

To customize styling now, use the `apply` prop on components to override default styles.

## Dependencies

- `dominator` - DOM framework
- `futures-signals` - Reactive state
- `dwind` / `dwind-macros` - CSS utilities
- `serde` - Serialization
- `web-sys` - Browser APIs
