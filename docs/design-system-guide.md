# Building a Design System with the DWIND Stack

A practical guide to creating a themeable, reactive component library using dominator, dwind, futures-signals, and friends. Based on lessons learned building `dwui` and `dwind-glass`.

---

## The Stack

| Layer | Crate | Role |
|-------|-------|------|
| DOM rendering | `dominator` | Zero-cost DOM builder with reactive bindings |
| Utility classes | `dwind` / `dwind-macros` | Tailwind-style CSS utilities compiled to Rust |
| CSS codegen | `dominator-css-bindgen` | Parses CSS at build time, generates Rust bindings |
| Reactivity | `futures-signals` | Fine-grained signals and mutables |
| Components | `futures-signals-component-macro` | `#[component]` proc macro for prop generation |

Everything compiles into the WASM binary. No runtime CSS parsing, no class string matching. Rust's dead code elimination acts as automatic tree-shaking.

---

## Crate Structure

A design system lives in its own workspace crate, independent of the application.

```
crates/my-design-system/
├── Cargo.toml
├── build.rs                    # CSS codegen
├── resources/css/
│   ├── tokens.css              # Utility classes referencing your CSS vars
│   └── base.css                # Resets, focus rings, shared styles
└── src/
    ├── lib.rs                  # Crate root, prelude, stylesheet init
    ├── theme/
    │   └── mod.rs              # Theme struct, CSS variable generation, apply fn
    ├── mixins/
    │   └── mod.rs              # Reusable DomBuilder transforms
    ├── input/
    │   └── mod.rs              # Shared traits (validation, value wrappers)
    └── components/
        ├── mod.rs              # Module tree + prelude re-exports
        └── core/
            ├── mod.rs
            ├── button.rs
            └── ...
```

### Cargo.toml

```toml
[dependencies]
dominator = { workspace = true }
dwind = { path = "../dwind" }
dwind-macros = { path = "../dwind-macros" }
futures-signals = { workspace = true }
futures-signals-component-macro = { workspace = true }
web-sys = { workspace = true }
wasm-bindgen = { workspace = true }

[build-dependencies]
dominator-css-bindgen = { path = "../dominator-css-bindgen" }
```

### build.rs

The build script converts CSS files into Rust modules at compile time:

```rust
use dominator_css_bindgen::css::generate_rust_bindings_from_file;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let css_dir = PathBuf::from("resources/css");

    generate_rust_bindings_from_file(
        &css_dir.join("tokens.css"),
        &out_dir.join("tokens.rs"),
    );

    println!("cargo:rerun-if-changed=resources/css/");
}
```

Then in your theme module:

```rust
pub mod tokens_css {
    include!(concat!(env!("OUT_DIR"), "/tokens.rs"));
}
```

Each generated module has an `apply_*_stylesheet()` function that injects the CSS into the document.

### lib.rs and Macro Scoping

This is critical:

```rust
#[macro_use]
extern crate dwind_macros;  // Required for dwclass! / dwclass_signal!

pub mod components;
pub mod theme;
// ...

pub mod prelude {
    pub use crate::components::prelude::*;
    pub use crate::theme::{apply_my_theme, MyTheme};
    // ...
}
```

**Consumer crates** must import macros explicitly:

```rust
#[macro_use]
extern crate dwind_macros;       // for dwclass!

#[macro_use]
extern crate my_design_system;   // for component macros (glass_button!, etc.)
```

The `#[component(render_fn = name)]` macro generates `#[macro_export] macro_rules!` with the snake_case name. These are crate-level macros, so consumers need `#[macro_use] extern crate`.

---

## Theme System

### CSS Custom Properties

The foundation of a themeable design system is CSS custom properties (`--your-prefix-*`). Define every visual token as a variable:

```rust
pub struct MyTheme {
    pub accent: String,
    pub bg: String,
    pub text_primary: String,
    pub border_radius: String,
    pub shadow: String,
    pub transition: String,
    // ...
}
```

Apply them to `:root` using dominator's `stylesheet!` macro:

```rust
pub fn apply_my_theme(theme: Option<MyTheme>) {
    let theme = theme.unwrap_or_default();

    stylesheet!(":root", {
        .raw(&theme.to_css_vars())
    });

    // Also apply your generated utility stylesheets
    tokens_css::apply_tokens_stylesheet();
}
```

### Runtime Theme Switching

To switch themes at runtime without appending extra `<style>` elements, set CSS properties directly on the document root:

```rust
fn apply_theme_to_root(theme: &MyTheme) {
    let raw = theme.to_css_vars(); // "--my-blur: 8px;--my-bg: rgba(...);"
    let root = web_sys::window().unwrap()
        .document().unwrap()
        .document_element().unwrap();
    let style = root.unchecked_ref::<web_sys::HtmlElement>().style();

    for decl in raw.split(';') {
        let decl = decl.trim();
        if let Some((prop, val)) = decl.split_once(':') {
            let _ = style.set_property(prop.trim(), val.trim());
        }
    }
}
```

All components referencing `var(--my-*)` update instantly.

### CSS Token Files

Your `resources/css/tokens.css` defines utility classes that map to your CSS variables:

```css
.my-text-primary { color: var(--my-text-primary); }
.my-text-secondary { color: var(--my-text-secondary); }
.my-bg-accent { background-color: var(--my-accent); }
```

These get compiled to Rust constants by `build.rs`, so you can use them with `dwclass!("my-text-primary")`.

---

## Component Pattern

### Declaration

```rust
use futures_signals_component_macro::component;

#[component(render_fn = my_button)]
struct MyButton {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[default(Box::new(|_: events::Click| {}))]
    on_click: dyn Fn(events::Click) -> () + 'static,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(ButtonVariant::Default)]
    variant: ButtonVariant,
}
```

This generates:
- `MyButtonProps` struct with builder pattern
- `my_button!({ .content(...).disabled(...) })` macro for ergonomic usage
- Each `#[signal]` field gets both a static setter and a `_signal` setter

### Render Function

```rust
pub fn my_button(props: MyButtonProps) -> Dom {
    let MyButtonProps { content, on_click, disabled, variant, apply } = props;

    // Broadcast signals used in multiple places
    let variant = variant.broadcast();
    let disabled = disabled.broadcast();

    html!("button", {
        .dwclass!("px-4 py-2 font-medium cursor-pointer transition-all")

        // Variant-based styling via signals
        .style_signal("background", variant.signal().map(|v| match v {
            ButtonVariant::Default => "var(--my-bg)",
            ButtonVariant::Accent  => "var(--my-accent)",
        }))

        // Disabled state
        .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
        .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))
        .attr_signal("disabled", disabled.signal().map(|d| if d { Some("disabled") } else { None }))

        .child_signal(content)

        .event(move |e: events::Click| { (on_click)(e); })

        // Extension point — consumers can customize the root element
        .apply_if(apply.is_some(), move |b| b.apply(apply.unwrap()))
    })
}
```

### Prop Rules

| Category | Pattern | Example |
|----------|---------|---------|
| Visual state | `#[signal]` — always reactive | `variant`, `disabled`, `size` |
| Content | `#[signal]` with `Option<Dom>` | `content`, `header`, `label` |
| Callbacks | Static `Box<dyn Fn(...)>` | `on_click`, `on_close`, `on_submit` |
| Values | Trait object or `Mutable` wrapper | `value: dyn InputValueWrapper` |
| Extension | Auto-provided by `#[component]` | `apply` field |

---

## Signals and Reactivity

### Broadcast for Multiple Consumers

A signal can only be consumed once (`.map()` takes ownership). Use `.broadcast()` when you need it in multiple places:

```rust
let variant = variant.broadcast();

// Now you can call .signal() multiple times
.style_signal("background", variant.signal().map(|v| ...))
.style_signal("color", variant.signal().map(|v| ...))
```

### map_ref! for Combining Signals

When a style depends on multiple signals:

```rust
use futures_signals::map_ref;

.style_signal("box-shadow", {
    let is_valid = is_valid.clone();
    let is_focused = is_focused.clone();
    map_ref! {
        let valid = is_valid.signal(),
        let focused = is_focused.signal() => {
            if !*valid {
                "var(--my-shadow-error)"
            } else if *focused {
                "var(--my-shadow-focus)"
            } else {
                "var(--my-shadow)"
            }
        }
    }
})
```

### Trait Objects as Component Fields

When a component accepts `dyn SomeTrait`, you need a Box delegation impl:

```rust
pub trait ToggleValue {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool>;
    fn toggle(&self);
}

impl ToggleValue for Mutable<bool> {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool> {
        self.signal().boxed_local()
    }
    fn toggle(&self) {
        self.set(!self.get());
    }
}

// Required so Box<dyn ToggleValue> works as the component field type
impl<T: ToggleValue + ?Sized> ToggleValue for Box<T> {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool> { (**self).get_signal() }
    fn toggle(&self) { (**self).toggle() }
}
```

Without the `Box<T>` impl, the component macro's generated code won't compile because the field is stored as `Box<dyn ToggleValue>`.

---

## Mixins

Reusable DomBuilder transforms for shared visual effects:

```rust
pub fn glass_surface(level: SurfaceLevel)
    -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    move |b| {
        b.style("background", match level {
                SurfaceLevel::Base => "var(--my-bg)",
                SurfaceLevel::Elevated => "var(--my-bg-elevated)",
            })
            .style("box-shadow", "var(--my-shadow)")
            .style("border-radius", "var(--my-border-radius)")
    }
}

// Usage:
html!("div", {
    .apply(glass_surface(SurfaceLevel::Elevated))
    .child(...)
})
```

---

## Tips, Tricks, and Gotchas

### Dominator Style Panics

Dominator's `set_style` panics in debug builds when:

1. **Empty string value** — `style_signal` must never return `""`. Every branch must return a valid CSS value.

```rust
// BAD — panics when size isn't Small
.style_signal("border-radius", size.signal().map(|s| match s {
    Size::Small => "4px",
    _ => "",  // PANIC!
}))

// GOOD — cover all variants
.style_signal("border-radius", size.signal().map(|s| match s {
    Size::Small => "4px",
    Size::Medium => "8px",
    Size::Large => "12px",
}))
```

2. **Unsupported CSS property names** — if the browser rejects a property name, dominator panics.

For vendor-prefixed properties, use the `MultiStr` array syntax:

```rust
// BAD — panics if browser doesn't support -webkit- prefix
.style("-webkit-backdrop-filter", "blur(8px)")

// GOOD — tries each name, succeeds if any works
.style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(8px)")
```

### Don't Return `None` from `style_signal`

`style_signal` expects the signal to return a value, not `Option`. Use `attr_signal` with `Option` for attributes, but for styles always return a string.

### Conditional DOM vs CSS Visibility

Prefer CSS visibility over conditional DOM creation when signals are involved:

```rust
// PROBLEMATIC — content signal is consumed when DOM is created,
// destroyed when open becomes false, and can't be recreated
.child_signal(open.signal().map(move |is_open| {
    if is_open { Some(panel_with_content_signal) } else { None }
}))

// BETTER — panel is always in DOM, CSS controls visibility
.child(html!("div", {
    .style_signal("opacity", open.signal().map(|o| if o { "1" } else { "0" }))
    .style_signal("pointer-events", open.signal().map(|o| if o { "auto" } else { "none" }))
    .child_signal(content)  // consumed once, lives forever
}))
```

This matters for modals, drawers, and any panel with reactive content.

### Signal `.map()` Consumes — Use `.broadcast()`

```rust
let disabled = disabled.broadcast();

// Now you can derive multiple signals from it
.style_signal("opacity", disabled.signal().map(...))
.attr_signal("disabled", disabled.signal().map(...))
.attr_signal("aria-disabled", disabled.signal().map(...))
```

If you forget `.broadcast()` and try to use a signal twice, you'll get a move error.

### `Box<dyn Fn()>` is NOT Clone

Wrap callbacks in `Rc` if you need to use them in multiple closures:

```rust
let on_close = std::rc::Rc::new(on_close);

// Now you can clone the Rc into multiple event handlers
.event({
    let on_close = on_close.clone();
    move |_: events::Click| { (on_close)(); }
})
.global_event({
    let on_close = on_close.clone();
    move |e: events::KeyDown| {
        if e.key() == "Escape" { (on_close)(); }
    }
})
```

### Never `return` Inside `map_ref!`

The `map_ref!` macro expands to code where `return` would exit the wrong scope, causing a type mismatch with `Poll`. Use `if/else` expressions instead:

```rust
// BAD
map_ref! {
    let a = sig_a,
    let b = sig_b => {
        if *a { return "yes"; }  // BREAKS
        "no"
    }
}

// GOOD
map_ref! {
    let a = sig_a,
    let b = sig_b => {
        if *a { "yes" } else { "no" }
    }
}
```

### CSS Class Naming Conventions

dwind uses Tailwind-compact style for spacing: `px-2`, `py-3`, `mb-4` (not `p-x-2`, `m-b-4`). Check `crates/dwind/src/modules/spacing.rs` or the class reference if unsure.

### Overflow Hidden for Rounded Containers

When a container has `border-radius` and child elements with backgrounds, always add `overflow: hidden` — otherwise child backgrounds will poke through the rounded corners:

```rust
html!("div", {
    .style("border-radius", "var(--my-radius-xl)")
    .style("overflow", "hidden")  // clips children to rounded shape
    .child(header_with_background)
    .child(body)
})
```

### Glass/Surface Visual Depth

Flat semi-transparent backgrounds look like colored rectangles, not glass. Add depth with:

1. **Bevelled edge highlight** — an `inset 0 0.5px 0 0 rgba(255,255,255,0.1)` box-shadow simulates light catching the top edge of a glass surface.

2. **Light gradient overlay** — layer a `linear-gradient(to bottom, rgba(255,255,255,0.06), transparent)` on top of the flat background to simulate overhead lighting.

3. **No hard borders** — use box-shadow rings (e.g., `0 0 0 2px var(--accent-muted)`) for focus/error states instead of `border-color` changes. Shadows are anti-aliased and feel softer.

```rust
// Bake the bevel into your shadow tokens:
shadow: "inset 0 0.5px 0 0 rgba(255,255,255,0.1), 0 4px 16px rgba(0,0,0,0.15)"

// Light gradient on cards:
.style("background", "\
    linear-gradient(to bottom, rgba(255,255,255,0.06), transparent 50%), \
    var(--my-bg-elevated)")

// Focus ring via box-shadow, not border:
.style_signal("box-shadow", is_focused.signal().map(|f| {
    if f { "var(--my-shadow-inset), 0 0 0 2px var(--my-accent-muted)" }
    else { "var(--my-shadow-inset)" }
}))
```

### `<label>` Doesn't Have `:disabled`

HTML `<label>` elements don't support the `:disabled` pseudo-class. If your component's root is a `<label>` (common for toggles/checkboxes), implement disabled state with explicit style signals:

```rust
.style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
.style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))
```

### Structural Dividers vs Surface Borders

In a design system, distinguish between:

- **Surface borders** (card edges, button outlines) — these define the shape of a glass surface. Replace with bevelled shadows for a softer look.
- **Structural dividers** (card header/footer separators, table row borders, sidebar edges) — these separate content regions. Keep them, but make them very subtle: `1px solid rgba(255,255,255,0.06)`.

---

## Example App Setup

A showcase/demo app should use your own design system for its chrome (navbar, layout) — eat your own dogfood.

```
examples/my-showcase/
├── Cargo.toml
├── Trunk.toml          # port, public dir
├── public/index.html   # background gradient, <link data-trunk rel="rust">
└── src/
    ├── lib.rs          # wasm_bindgen(start), init stylesheets + theme
    ├── router.rs       # hash-based routing
    └── pages/
        └── ...
```

**index.html** — set a background that makes glass effects visible:

```html
<style>
    html, body {
        margin: 0; padding: 0; min-height: 100vh;
        background: linear-gradient(135deg, #080614 0%, #1a1540 50%, #12101e 100%);
        background-attachment: fixed;
    }
</style>
```

**lib.rs**:

```rust
#[macro_use] extern crate dwind_macros;
#[macro_use] extern crate my_design_system;

#[wasm_bindgen(start)]
pub async fn main() {
    dwind::stylesheet();                    // base utility classes
    my_design_system::apply_my_theme(None); // design system theme + token styles
    dominator::append_dom(&dominator::body(), app());
}
```

The `wasm-bindgen-futures` crate is required for `pub async fn main()`.

---

## Checklist

- [ ] Crate compiles: `cargo build -p my-design-system`
- [ ] WASM target works: `cargo build -p my-design-system --target wasm32-unknown-unknown`
- [ ] Consumer crate has `#[macro_use] extern crate` for both `dwind_macros` and your crate
- [ ] All `style_signal` branches return non-empty strings
- [ ] Vendor-prefixed CSS uses array syntax: `["prop", "-webkit-prop"]`
- [ ] Signals used in multiple places are `.broadcast()`ed
- [ ] `Box<dyn Trait>` fields have delegation impls
- [ ] Callbacks used in multiple closures are wrapped in `Rc`
- [ ] Theme tokens use a consistent `--prefix-*` namespace
- [ ] Showcase app background makes glass/transparency effects visible
