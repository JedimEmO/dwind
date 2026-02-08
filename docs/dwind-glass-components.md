# dwind-glass Component Library Reference

A glassmorphic design system for DOMINATOR. All components use frosted-glass surfaces, `--glass-*` CSS custom properties, and reactive signals.

## Setup

```rust
#[macro_use] extern crate dwind_macros;
#[macro_use] extern crate dwind_glass;

use dwind_glass::prelude::*;
use futures_signals::signal::Mutable;
```

Initialize theme at startup:

```rust
dwind::stylesheet();
apply_glass_theme(None); // or Some(GlassTheme { ... })
```

---

## Core Components

### Button

```rust
glass_button!({
    .content(Some(text("Click me")))
    .variant(ButtonVariant::Glass)  // Glass, Filled, Ghost, Danger
    .size(ButtonSize::Medium)       // Small, Medium, Large
    .on_click(Box::new(|_: events::Click| { /* ... */ }))
    .disabled(false)
    .loading(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `content` / `content_signal` | `Option<Dom>` | `None` | Button content |
| `variant` / `variant_signal` | `ButtonVariant` | `Glass` | Visual style |
| `size` / `size_signal` | `ButtonSize` | `Medium` | Button size |
| `on_click` | `Box<dyn Fn(events::Click)>` | no-op | Click handler |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `loading` / `loading_signal` | `bool` | `false` | Loading spinner |

**Interactivity:** Hover glow, active press scale(0.98), keyboard focus ring.

---

### Card

```rust
glass_card!({
    .variant(CardVariant::Default)  // Default, Elevated, Inset
    .content(Some(html!("div", { .text("Card body") })))
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` / `variant_signal` | `CardVariant` | `Default` | Surface depth |
| `content` / `content_signal` | `Option<Dom>` | `None` | Card body |

**Interactivity:** Hover elevates shadow on Default and Elevated variants.

---

### Badge

```rust
glass_badge!({
    .content(Some(text("New")))
    .variant(BadgeVariant::Accent)  // Default, Success, Warning, Error, Info, Accent
})
```

---

### Text Input

```rust
let value = Mutable::new(String::new());

glass_text_input!({
    .value(Box::new(value.clone()))
    .label("Email".to_string())
    .placeholder("you@example.com".to_string())
    .input_type(TextInputType::Text)  // Text or Password
    .disabled(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Box<dyn InputValueWrapper>` | `Mutable<String>` | Bound value |
| `label` / `label_signal` | `String` | `""` | Floating label |
| `placeholder` / `placeholder_signal` | `String` | `""` | Placeholder text |
| `input_type` / `input_type_signal` | `TextInputType` | `Text` | Input type |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `is_valid` / `is_valid_signal` | `ValidationResult` | `Valid` | Validation state |

**Interactivity:** Focus ring, hover border glow, invalid state red ring.

---

### Toggle

```rust
let enabled = Mutable::new(false);

glass_toggle!({
    .checked(Box::new(enabled.clone()))
    .label(Some("Enable feature".to_string()))
    .disabled(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Box<dyn GlassToggleValue>` | `Mutable<bool>` | Toggle state |
| `label` / `label_signal` | `Option<String>` | `None` | Label text |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |

---

### Checkbox

```rust
let accepted = Mutable::new(false);

glass_checkbox!({
    .checked(Box::new(accepted.clone()))
    .label(Some("I agree to the terms".to_string()))
    .disabled(false)
    .is_valid_signal(accepted.signal().map(|c| {
        if c { ValidationResult::Valid }
        else { ValidationResult::Invalid { message: "Required".into() } }
    }))
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Box<dyn GlassToggleValue>` | `Mutable<bool>` | Checked state |
| `label` / `label_signal` | `Option<String>` | `None` | Label text |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `is_valid` / `is_valid_signal` | `ValidationResult` | `Valid` | Validation state |

**Visual:** 18x18 rounded box, accent background when checked, animated checkmark. Error ring + message when invalid. ARIA `role="checkbox"`.

---

### Radio Group

```rust
use dwind_glass::input::GlassSelectOption;

let choice = Mutable::new(None::<String>);

glass_radio_group!({
    .value(Box::new(choice.clone()))
    .options(vec![
        GlassSelectOption { label: "Option A".into(), value: "a".into() },
        GlassSelectOption { label: "Option B".into(), value: "b".into() },
        GlassSelectOption { label: "Option C".into(), value: "c".into() },
    ])
    .disabled(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Box<dyn GlassSelectValue>` | `Mutable<Option<String>>` | Selected value |
| `options` | `Vec<GlassSelectOption>` | `vec![]` | Radio options |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `is_valid` / `is_valid_signal` | `ValidationResult` | `Valid` | Validation state |

**Visual:** Vertical list of 18x18 circles with animated inner dot. Error ring + message when invalid. ARIA `role="radiogroup"`.

---

### Select (Dropdown)

```rust
let framework = Mutable::new(None::<String>);

glass_select!({
    .value(Box::new(framework.clone()))
    .label("Framework".to_string())
    .placeholder("Choose one...".to_string())
    .options(vec![
        GlassSelectOption { label: "Dominator".into(), value: "dominator".into() },
        GlassSelectOption { label: "Leptos".into(), value: "leptos".into() },
    ])
    .disabled(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Box<dyn GlassSelectValue>` | `Mutable<Option<String>>` | Selected value |
| `options` | `Vec<GlassSelectOption>` | `vec![]` | Dropdown options |
| `label` / `label_signal` | `String` | `""` | Label text |
| `placeholder` / `placeholder_signal` | `String` | `"Select..."` | Placeholder |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `is_valid` / `is_valid_signal` | `ValidationResult` | `Valid` | Validation state |

**Visual:** Click-to-open trigger with chevron, elevated glass dropdown panel, selected item highlighted. Error ring + red label + message when invalid. Closes on click-outside, Escape, or selection. ARIA `listbox` pattern.

---

### Combobox (Searchable Select)

```rust
let country = Mutable::new(None::<String>);

glass_combobox!({
    .value(Box::new(country.clone()))
    .label("Country".to_string())
    .placeholder("Search...".to_string())
    .options(vec![
        GlassSelectOption { label: "United States".into(), value: "us".into() },
        GlassSelectOption { label: "United Kingdom".into(), value: "uk".into() },
    ])
    .disabled(false)
})
```

**Props:** Same as Select.

**Props:** Same as Select (including `is_valid`).

**Differences from Select:** Trigger is a text input — typing filters options with case-insensitive substring matching. Opens on focus, closes on blur (150ms delay for option click). Shows "No results" when filter matches nothing.

---

### Modal

```rust
let open = Mutable::new(false);

glass_modal!({
    .open_signal(open.signal())
    .on_close({
        let open = open.clone();
        Box::new(move || open.set(false))
    })
    .title(Some("Confirm".to_string()))
    .size(ModalSize::Medium)  // Small, Medium, Large, Full
    .content(Some(html!("div", { .text("Are you sure?") })))
})
```

---

## Navigation Components

### Navbar

```rust
glass_navbar!({
    .brand(Some(html!("span", { .text("My App") })))
    .items(vec![/* nav link Doms */])
    .trailing(Some(html!("span", { .text("User") })))
    .sticky(true)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `brand` / `brand_signal` | `Option<Dom>` | `None` | Left branding area |
| `items` | `Vec<Dom>` (signal_vec) | `vec![]` | Center navigation items |
| `trailing` / `trailing_signal` | `Option<Dom>` | `None` | Right side content |
| `sticky` / `sticky_signal` | `bool` | `true` | Sticky to top |

---

### Sidebar

```rust
let collapsed = Mutable::new(false);

glass_sidebar!({
    .collapsed_signal(collapsed.signal())
    .header(Some(html!("span", { .text("Menu") })))
    .items(vec![
        glass_nav_item!({
            .label("Dashboard".to_string())
            .icon(Some(html!("span", { .text("\u{25A3}") })))
            .active(true)
            .collapsed_signal(collapsed.signal())
            .on_click(Box::new(|| {}))
        }),
    ])
    .footer(Some(html!("span", { .text("v1.0") })))
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<Dom>` (signal_vec) | `vec![]` | Navigation items |
| `header` / `header_signal` | `Option<Dom>` | `None` | Top section |
| `footer` / `footer_signal` | `Option<Dom>` | `None` | Bottom section |
| `collapsed` / `collapsed_signal` | `bool` | `false` | Collapse to 4rem |

---

### Nav Item

Reusable navigation item for sidebars and vertical menus. Supports icon, label, badge with collapse-aware layout.

```rust
glass_nav_item!({
    .label("Settings".to_string())
    .icon(Some(html!("span", { .text("\u{2699}") })))
    .badge(Some(glass_badge!({ .content(Some(text("3"))) })))
    .active_signal(is_active.signal())
    .collapsed_signal(sidebar_collapsed.signal())
    .on_click(Box::new(|| { /* navigate */ }))
    .disabled(false)
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `label` | `String` | `""` | Item text |
| `icon` | `Option<Dom>` | `None` | Left icon (visible when collapsed) |
| `badge` | `Option<Dom>` | `None` | Right badge (hidden when collapsed) |
| `active` / `active_signal` | `bool` | `false` | Active highlight |
| `disabled` / `disabled_signal` | `bool` | `false` | Disabled state |
| `on_click` | `Box<dyn Fn()>` | no-op | Click handler |
| `collapsed` / `collapsed_signal` | `bool` | `false` | Collapse mode (icon only) |

**Visual:** Rounded item, accent-muted bg when active, hover highlight. When collapsed: label+badge fade out, icon centers.

---

### Tabs

Underline-style tab navigation with accent indicator.

```rust
glass_tabs!({
    .tabs(vec![
        TabItem { label: "First".into(), content: html!("div", { .text("...") }) },
        TabItem { label: "Second".into(), content: html!("div", { .text("...") }) },
    ])
    .active_index(Box::new(Mutable::new(0usize)))
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tabs` | `Vec<TabItem>` | `vec![]` | Tab definitions |
| `active_index` | `Box<dyn GlassTabIndex>` | `Mutable<0usize>` | Active tab index |

**Visual:** Bottom border with 2px accent underline on active tab. Hover shows subtle border. ARIA `tablist`/`tab`/`tabpanel`.

---

### Dropdown Menu

```rust
glass_menu!({
    .trigger(Some(glass_button!({
        .content(Some(text("Actions")))
        .variant(ButtonVariant::Glass)
    })))
    .items(vec![
        GlassMenuEntry::Item(GlassMenuItem {
            label: "Edit".into(),
            icon: Some(html!("span", { .text("\u{270F}") })),
            on_click: Box::new(|| {}),
            disabled: false,
            variant: MenuItemVariant::Default,
        }),
        GlassMenuEntry::Divider,
        GlassMenuEntry::Item(GlassMenuItem {
            label: "Delete".into(),
            icon: None,
            on_click: Box::new(|| {}),
            disabled: false,
            variant: MenuItemVariant::Danger,
        }),
    ])
})
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `trigger` | `Option<Dom>` | `None` | Element that opens the menu |
| `items` | `Vec<GlassMenuEntry>` | `vec![]` | Menu entries (Item or Divider) |

**GlassMenuItem fields:** `label: String`, `icon: Option<Dom>`, `on_click: Box<dyn Fn()>`, `disabled: bool`, `variant: MenuItemVariant` (Default/Danger).

**Visual:** Elevated glass dropdown with rounded item highlights. Danger items in red. Closes on click-outside, Escape, or item click. ARIA `menu`/`menuitem`.

---

### Breadcrumbs

```rust
glass_breadcrumbs!({
    .items(vec![
        BreadcrumbItem { label: "Home".into(), on_click: Some(Box::new(|| {})) },
        BreadcrumbItem { label: "Settings".into(), on_click: None },
    ])
})
```

---

## Data Display Components

### Table

```rust
glass_table!({
    .columns(vec!["Name".into(), "Role".into()])
    .rows(vec![
        vec!["Alice".into(), "Admin".into()],
        vec!["Bob".into(), "User".into()],
    ])
    .striped(true)
})
```

### Stat Card

```rust
glass_stat_card!({
    .label("Revenue".to_string())
    .value("$12,345".to_string())
})
```

### Tooltip

```rust
glass_text_tooltip!({
    .text("Helpful hint".to_string())
    .trigger(Some(html!("span", { .text("Hover me") })))
})
```

### Avatar & Progress Bar

```rust
glass_avatar!({ .src("avatar.png".to_string()) })
glass_progress_bar!({ .value(0.75) })
```

---

## Value Traits

Components that bind to reactive state use trait objects:

| Trait | Backing type | Components |
|-------|-------------|------------|
| `InputValueWrapper` | `Mutable<T: FromStr+ToString>` | TextInput |
| `GlassToggleValue` | `Mutable<bool>` | Toggle, Checkbox |
| `GlassSelectValue` | `Mutable<Option<String>>` | Select, Radio Group, Combobox |

Pass as `Box::new(my_mutable.clone())`:

```rust
let value = Mutable::new(None::<String>);
glass_select!({ .value(Box::new(value.clone())) ... })
```

## Shared Types

```rust
use dwind_glass::input::GlassSelectOption;

GlassSelectOption {
    label: "Display text".to_string(),
    value: "internal_key".to_string(),
}
```

Used by Select, Radio Group, and Combobox.

---

## Theming

```rust
use dwind_glass::prelude::*;

// Default dark glass theme
apply_glass_theme(None);

// Custom theme
apply_glass_theme(Some(GlassTheme {
    accent: "#6366f1".into(),
    // ...
}));
```

All components reference `--glass-*` CSS custom properties. Switch themes at runtime by re-calling `apply_glass_theme()` or setting properties on `:root`.
