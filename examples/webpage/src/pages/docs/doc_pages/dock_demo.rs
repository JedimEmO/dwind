//! Dock demo page - showcases the dwind-dock dockable views component.

use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use dominator::{clone, Dom};
use dwind::prelude::*;
use dwind_dock::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use example_html_highlight_macro::example_html;
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::Arc;

/// Available workspace presets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Workspace {
    Default,
    Coding,
    Debug,
    Minimal,
}

impl Workspace {
    fn label(&self) -> &'static str {
        match self {
            Workspace::Default => "Default",
            Workspace::Coding => "Coding",
            Workspace::Debug => "Debug",
            Workspace::Minimal => "Minimal",
        }
    }

    fn create_layout(&self) -> DockLayout {
        match self {
            Workspace::Default => create_default_layout(),
            Workspace::Coding => create_coding_layout(),
            Workspace::Debug => create_debug_layout(),
            Workspace::Minimal => create_minimal_layout(),
        }
    }
}

/// The dock demo page.
pub fn dock_demo_page() -> Dom {
    let current_workspace = Mutable::new(Workspace::Default);
    let state = Arc::new(DockState::new(Workspace::Default.create_layout(), |_layout| {
        // Could persist to localStorage here
    }));

    html!("div", {
        .dwclass!("w-full h-full flex flex-col")
        .child(doc_page_title("Dock Demo"))
        .child(html!("p", {
            .dwclass!("text-gray-400 mb-4")
            .text("A Blender-style dockable panel system. Drag dividers to resize panes, click tabs to switch content, close tabs with ×.")
        }))

        // Workspace selector toolbar
        .child(html!("div", {
            .dwclass!("flex flex-row gap-2 mb-4 items-center")
            .child(html!("span", {
                .dwclass!("text-gray-400 text-sm")
                .text("Workspace:")
            }))
            .children([
                Workspace::Default,
                Workspace::Coding,
                Workspace::Debug,
                Workspace::Minimal,
            ].into_iter().map({
                let current_workspace = current_workspace.clone();
                let state = state.clone();
                move |ws| workspace_button(ws, current_workspace.clone(), state.clone())
            }))
        }))

        // Demo area - full width, generous height
        .child(html!("div", {
            .dwclass!("w-full rounded-lg overflow-hidden border border-gray-700 mb-6")
            .style("height", "70vh")
            .style("min-height", "500px")
            .child(create_dock_with_state(state.clone()))
        }))

        // Code example
        .child(code(&CREATE_DEFAULT_LAYOUT_EXAMPLE_HTML_MAP))
    })
}

fn workspace_button(workspace: Workspace, current: Mutable<Workspace>, state: Arc<DockState>) -> Dom {
    let is_active = current.signal().map(move |c| c == workspace);

    button!({
        .apply(move |b| {
            dwclass!(b, "px-3 py-1 text-sm")
        })
        .content_signal(is_active.map(move |active| {
            let label = workspace.label();
            Some(html!("span", {
                .style("font-weight", if active { "bold" } else { "normal" })
                .text(label)
            }))
        }))
        .on_click(clone!(current, state => move |_| {
            current.set(workspace);
            state.set_layout(workspace.create_layout());
        }))
    })
}

fn create_dock_with_state(state: Arc<DockState>) -> Dom {
    let tab_content: TabContentRenderer = Arc::new(|tab_id| render_tab_content(tab_id));

    dock_area(DockAreaProps {
        state: (*state).clone(),
        tab_content,
        theme: DockTheme::default(),
        apply: None,
    })
}

// --- Workspace Layouts ---

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn create_default_layout() -> DockLayout {
    let left_panel = DockNode::Leaf {
        id: 1,
        container: TabContainer::new(vec![
            Tab::new("files", "Files"),
            Tab::new("properties", "Properties"),
        ]),
    };

    let right_panel = DockNode::Leaf {
        id: 2,
        container: TabContainer::new(vec![
            Tab::new("editor", "Editor"),
            Tab::new("preview", "Preview"),
        ]),
    };

    let top_split = DockNode::Split {
        id: 3,
        direction: SplitDirection::Horizontal,
        ratio: 0.3,
        first: Box::new(left_panel),
        second: Box::new(right_panel),
    };

    let bottom_panel = DockNode::Leaf {
        id: 4,
        container: TabContainer::new(vec![
            Tab::new("console", "Console"),
            Tab::new("output", "Output"),
            Tab::new("errors", "Errors"),
        ]),
    };

    let root = DockNode::Split {
        id: 5,
        direction: SplitDirection::Vertical,
        ratio: 0.7,
        first: Box::new(top_split),
        second: Box::new(bottom_panel),
    };

    DockLayout::new(root)
}

fn create_coding_layout() -> DockLayout {
    // Focused coding: large editor, small file browser, no bottom panel
    let files_panel = DockNode::Leaf {
        id: 10,
        container: TabContainer::new(vec![
            Tab::new("files", "Files"),
        ]),
    };

    let editor_panel = DockNode::Leaf {
        id: 11,
        container: TabContainer::new(vec![
            Tab::new("editor", "Editor"),
            Tab::new("editor2", "Editor 2"),
            Tab::new("editor3", "Editor 3"),
        ]),
    };

    let root = DockNode::Split {
        id: 12,
        direction: SplitDirection::Horizontal,
        ratio: 0.2,
        first: Box::new(files_panel),
        second: Box::new(editor_panel),
    };

    DockLayout::new(root)
}

fn create_debug_layout() -> DockLayout {
    // Debug layout: editor on left, debug panels on right, console at bottom
    let editor_panel = DockNode::Leaf {
        id: 20,
        container: TabContainer::new(vec![
            Tab::new("editor", "Editor"),
        ]),
    };

    let variables_panel = DockNode::Leaf {
        id: 21,
        container: TabContainer::new(vec![
            Tab::new("variables", "Variables"),
            Tab::new("watch", "Watch"),
        ]),
    };

    let callstack_panel = DockNode::Leaf {
        id: 22,
        container: TabContainer::new(vec![
            Tab::new("callstack", "Call Stack"),
            Tab::new("breakpoints", "Breakpoints"),
        ]),
    };

    let right_split = DockNode::Split {
        id: 23,
        direction: SplitDirection::Vertical,
        ratio: 0.5,
        first: Box::new(variables_panel),
        second: Box::new(callstack_panel),
    };

    let top_split = DockNode::Split {
        id: 24,
        direction: SplitDirection::Horizontal,
        ratio: 0.6,
        first: Box::new(editor_panel),
        second: Box::new(right_split),
    };

    let console_panel = DockNode::Leaf {
        id: 25,
        container: TabContainer::new(vec![
            Tab::new("console", "Console"),
            Tab::new("output", "Output"),
            Tab::new("errors", "Errors"),
        ]),
    };

    let root = DockNode::Split {
        id: 26,
        direction: SplitDirection::Vertical,
        ratio: 0.7,
        first: Box::new(top_split),
        second: Box::new(console_panel),
    };

    DockLayout::new(root)
}

fn create_minimal_layout() -> DockLayout {
    // Just an editor
    let editor_panel = DockNode::Leaf {
        id: 30,
        container: TabContainer::new(vec![
            Tab::new("editor", "Editor"),
        ]),
    };

    DockLayout::new(editor_panel)
}

// --- Tab Content Renderers ---

fn render_tab_content(tab_id: &TabId) -> Dom {
    match tab_id.as_str() {
        "files" => render_files_panel(),
        "properties" => render_properties_panel(),
        "editor" | "editor2" | "editor3" => render_editor_panel(),
        "preview" => render_preview_panel(),
        "console" => render_console_panel(),
        "output" => render_output_panel(),
        "errors" => render_errors_panel(),
        "variables" => render_variables_panel(),
        "watch" => render_watch_panel(),
        "callstack" => render_callstack_panel(),
        "breakpoints" => render_breakpoints_panel(),
        _ => html!("div", {
            .dwclass!("p-4 text-gray-500")
            .text(&format!("Content for: {}", tab_id))
        }),
    }
}

fn render_files_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-gray-300 text-sm h-full overflow-auto")
        .children([
            html!("div", {
                .dwclass!("font-bold mb-2 text-gray-200")
                .text("Project Files")
            }),
            file_tree_item("src/", true),
            file_tree_item("  lib.rs", false),
            file_tree_item("  main.rs", false),
            file_tree_item("  components/", true),
            file_tree_item("    mod.rs", false),
            file_tree_item("    button.rs", false),
            file_tree_item("Cargo.toml", false),
            file_tree_item("README.md", false),
        ])
    })
}

fn file_tree_item(name: &str, is_folder: bool) -> Dom {
    let icon = if is_folder { "📁 " } else { "📄 " };
    let color = if is_folder { "rgb(250, 204, 21)" } else { "rgb(156, 163, 175)" };

    html!("div", {
        .dwclass!("py-1 px-2 hover:bg-gray-700 rounded cursor-pointer")
        .child(html!("span", {
            .style("color", color)
            .text(icon)
        }))
        .text(name.trim())
    })
}

fn render_properties_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-gray-300 text-sm")
        .children([
            html!("div", {
                .dwclass!("font-bold mb-2 text-gray-200")
                .text("Properties")
            }),
            property_row("Name", "Button"),
            property_row("Width", "120px"),
            property_row("Height", "40px"),
            property_row("Background", "#3B82F6"),
            property_row("Border Radius", "8px"),
        ])
    })
}

fn property_row(label: &str, value: &str) -> Dom {
    html!("div", {
        .dwclass!("flex justify-between py-1 border-b border-gray-700")
        .children([
            html!("span", { .dwclass!("text-gray-500") .text(label) }),
            html!("span", { .dwclass!("text-gray-300") .text(value) }),
        ])
    })
}

fn render_editor_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 font-mono text-sm bg-gray-950 h-full overflow-auto")
        .children([
            code_line(1, "use dominator::Dom;"),
            code_line(2, "use dwind::prelude::*;"),
            code_line(3, ""),
            code_line(4, "pub fn button() -> Dom {"),
            code_line(5, "    html!(\"button\", {"),
            code_line(6, "        .dwclass!(\"px-4 py-2\")"),
            code_line(7, "        .dwclass!(\"bg-blue-500\")"),
            code_line(8, "        .text(\"Click me\")"),
            code_line(9, "    })"),
            code_line(10, "}"),
        ])
    })
}

fn code_line(num: i32, text: &str) -> Dom {
    html!("div", {
        .dwclass!("flex")
        .children([
            html!("span", {
                .dwclass!("w-8 text-gray-600 text-right pr-3 select-none")
                .text(&num.to_string())
            }),
            html!("span", {
                .dwclass!("text-gray-300")
                .text(text)
            }),
        ])
    })
}

fn render_preview_panel() -> Dom {
    html!("div", {
        .dwclass!("p-4 flex items-center justify-center h-full bg-gray-950")
        .child(html!("button", {
            .dwclass!("px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600")
            .text("Click me")
        }))
    })
}

fn render_console_panel() -> Dom {
    html!("div", {
        .dwclass!("p-2 font-mono text-xs bg-gray-950 h-full text-gray-400 overflow-auto")
        .children([
            console_line("[info]", "Application started", "text-blue-400"),
            console_line("[info]", "Loading configuration...", "text-blue-400"),
            console_line("[info]", "Config loaded successfully", "text-green-400"),
            console_line("[warn]", "Deprecated API usage detected", "text-yellow-400"),
            console_line("[info]", "Server listening on port 8080", "text-blue-400"),
        ])
    })
}

fn console_line(prefix: &str, message: &str, prefix_class: &str) -> Dom {
    html!("div", {
        .dwclass!("py-0.5")
        .children([
            html!("span", {
                .attr("class", prefix_class)
                .text(prefix)
                .text(" ")
            }),
            html!("span", {
                .text(message)
            }),
        ])
    })
}

fn render_output_panel() -> Dom {
    html!("div", {
        .dwclass!("p-2 font-mono text-xs bg-gray-950 h-full text-gray-300")
        .children([
            html!("div", { .text("Compiling dwind-dock v0.1.0") }),
            html!("div", { .text("   Finished dev [unoptimized] target(s) in 0.42s") }),
            html!("div", { .text("    Running `target/debug/example`") }),
        ])
    })
}

fn render_errors_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-sm")
        .child(html!("div", {
            .dwclass!("text-green-400 flex items-center gap-2")
            .text("✓ No errors")
        }))
    })
}

fn render_variables_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-sm font-mono")
        .children([
            html!("div", {
                .dwclass!("font-bold mb-2 text-gray-200")
                .text("Local Variables")
            }),
            variable_row("counter", "i32", "42"),
            variable_row("name", "String", "\"hello\""),
            variable_row("items", "Vec<i32>", "[1, 2, 3]"),
        ])
    })
}

fn variable_row(name: &str, typ: &str, value: &str) -> Dom {
    html!("div", {
        .dwclass!("flex gap-2 py-1 border-b border-gray-700 text-xs")
        .children([
            html!("span", { .dwclass!("text-blue-400") .text(name) }),
            html!("span", { .dwclass!("text-gray-500") .text(typ) }),
            html!("span", { .dwclass!("text-green-400") .text(value) }),
        ])
    })
}

fn render_watch_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-sm text-gray-400")
        .text("Add expressions to watch...")
    })
}

fn render_callstack_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-sm font-mono")
        .children([
            html!("div", {
                .dwclass!("font-bold mb-2 text-gray-200")
                .text("Call Stack")
            }),
            stack_frame("main", "src/main.rs", 15, true),
            stack_frame("process", "src/lib.rs", 42, false),
            stack_frame("calculate", "src/math.rs", 8, false),
        ])
    })
}

fn stack_frame(func: &str, file: &str, line: i32, is_current: bool) -> Dom {
    let (bg_color, text_color) = if is_current {
        ("rgb(30, 58, 138)", "rgb(191, 219, 254)") // blue-900, blue-200
    } else {
        ("transparent", "rgb(156, 163, 175)") // gray-400
    };

    html!("div", {
        .dwclass!("py-1 px-2 rounded cursor-pointer text-xs hover:bg-gray-700")
        .style("background-color", bg_color)
        .style("color", text_color)
        .children([
            html!("div", { .dwclass!("text-gray-200") .text(func) }),
            html!("div", { .text(&format!("{}:{}", file, line)) }),
        ])
    })
}

fn render_breakpoints_panel() -> Dom {
    html!("div", {
        .dwclass!("p-3 text-sm font-mono")
        .children([
            html!("div", {
                .dwclass!("font-bold mb-2 text-gray-200")
                .text("Breakpoints")
            }),
            breakpoint_row("src/main.rs", 15, true),
            breakpoint_row("src/lib.rs", 42, true),
            breakpoint_row("src/math.rs", 8, false),
        ])
    })
}

fn breakpoint_row(file: &str, line: i32, enabled: bool) -> Dom {
    let dot_color = if enabled { "rgb(239, 68, 68)" } else { "rgb(75, 85, 99)" }; // red-500 / gray-600

    html!("div", {
        .dwclass!("flex items-center gap-2 py-1 text-xs")
        .children([
            html!("span", {
                .style("color", dot_color)
                .text("●")
            }),
            html!("span", {
                .dwclass!("text-gray-300")
                .text(&format!("{}:{}", file, line))
            }),
        ])
    })
}
