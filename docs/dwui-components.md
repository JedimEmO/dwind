# DWUI Component Library Reference

## Setup

```rust
use dwui::prelude::*;
use futures_signals::signal::Mutable;
```

## Button

```rust
button!({
    .content(Some(text("Click me")))
    .on_click(|_| println!("Clicked!"))
    .disabled(false)
    .button_type(ButtonType::Flat)  // or ButtonType::Border
    .apply(|b| dwclass!(b, "w-32"))
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `content` | `Option<Dom>` | Button content |
| `on_click` | `Fn(events::Click)` | Click handler |
| `disabled` / `disabled_signal` | `bool` | Disable state |
| `button_type` / `button_type_signal` | `ButtonType` | `Flat` or `Border` |
| `apply` | `FnOnce(DomBuilder)` | Custom styling |

## Modal

```rust
let show = Mutable::new(false);

modal!({
    .open_signal(show.signal())
    .on_close(clone!(show => move || show.set(false)))
    .size(ModalSize::Medium)
    .content(Some(html!("div", {
        .text("Modal content")
    })))
    .close_on_backdrop_click(true)
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `open` / `open_signal` | `bool` | Open state |
| `on_close` | `Fn()` | Close callback |
| `content` / `content_signal` | `Option<Dom>` | Modal body |
| `size` / `size_signal` | `ModalSize` | `Small`, `Medium`, `Large`, `Full` |
| `close_on_backdrop_click` | `bool` | Close on backdrop (default: true) |

## TextInput

```rust
let email = Mutable::new(String::new());

text_input!({
    .value(email.clone())
    .label("Email".to_string())
    .input_type(TextInputType::Text)  // or TextInputType::Password
    .claim_focus(true)
    .is_valid_signal(email.signal_ref(|v| {
        if v.contains("@") {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid { message: "Invalid email".into() }
        }
    }))
    .on_submit(|| println!("Submitted!"))
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `value` | `impl InputValueWrapper` | Value state (usually `Mutable<String>`) |
| `label` / `label_signal` | `String` | Input label |
| `input_type` / `input_type_signal` | `TextInputType` | `Text` or `Password` |
| `is_valid` / `is_valid_signal` | `ValidationResult` | Validation state |
| `claim_focus` | `bool` | Auto-focus on mount |
| `on_submit` | `FnMut()` | Enter key handler |

## Slider

```rust
let volume = Mutable::new(50.0f32);

slider!({
    .value(volume.clone())
    .min(0.0)
    .max(100.0)
    .step(5.0)
    .label("Volume".to_string())
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `value` | `impl InputValueWrapper` | Current value |
| `min` / `min_signal` | `f32` | Minimum (default: 0.0) |
| `max` / `max_signal` | `f32` | Maximum (default: 100.0) |
| `step` / `step_signal` | `f32` | Step size (default: 1.0) |
| `label` / `label_signal` | `String` | Label text |

## Select

```rust
let choice = Mutable::new(String::new());

select!({
    .value(choice.clone())
    .label("Choose option".to_string())
    .options(vec![
        ("a".to_string(), "Option A".to_string()),
        ("b".to_string(), "Option B".to_string()),
        ("c".to_string(), "Option C".to_string()),
    ])
    .is_valid_signal(choice.signal_ref(|v| {
        if v.is_empty() {
            ValidationResult::Invalid { message: "Required".into() }
        } else {
            ValidationResult::Valid
        }
    }))
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `value` | `impl InputValueWrapper` | Selected value |
| `options` / `options_signal_vec` | `Vec<(String, String)>` | (key, label) pairs |
| `label` / `label_signal` | `String` | Label text |
| `is_valid` / `is_valid_signal` | `ValidationResult` | Validation state |

## Card

```rust
card!({
    .scheme(ColorScheme::Void)  // or ColorScheme::Primary
    .apply(|b| {
        dwclass!(b, "p-4 w-64")
        .child(text("Card content"))
    })
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `scheme` / `scheme_signal` | `ColorScheme` | `Primary` or `Void` |
| `content` / `content_signal` | `Dom` | Card content |
| `apply` | `FnOnce(DomBuilder)` | Custom styling |

## Heading

```rust
heading!({
    .content(text("Page Title"))
    .text_size(TextSize::ExtraLarge)  // or Base, Large
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `content` / `content_signal` | `Dom` | Heading text |
| `text_size` / `text_size_signal` | `TextSize` | `Base`, `Large`, `ExtraLarge` |

## List

```rust
let selected = Mutable::new(None::<usize>);

pretty_list!({
    .items(vec![
        text("Item 1"),
        text("Item 2"),
        text("Item 3"),
    ])
    .selected_index_signal(selected.signal())
    .item_click_handler(clone!(selected => move |idx| {
        selected.set(Some(idx));
    }))
})
```

**Props:**
| Prop | Type | Description |
|------|------|-------------|
| `items` / `items_signal_vec` | `Vec<Dom>` | List items |
| `selected_index` / `selected_index_signal` | `Option<usize>` | Selected item index |
| `item_click_handler` | `Fn(usize)` | Click handler with index |

## Validation

```rust
enum ValidationResult {
    Valid,
    Invalid { message: String },
}

// Usage in components
.is_valid_signal(value.signal_ref(|v| {
    if is_valid(v) {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid { message: "Error message".into() }
    }
}))
```

## Theming

Apply a custom theme globally:

```rust
use dwui::theme::prelude::ColorsCssVariables;
use dwind::colors::DWIND_COLORS;

// Apply theme at startup
dwui::theme::apply_style_sheet(Some(ColorsCssVariables::new(
    &DWIND_COLORS["blue"],      // Primary
    &DWIND_COLORS["gray"],      // Text on primary
    &DWIND_COLORS["gray"],      // Void/background
    &DWIND_COLORS["red"],       // Error
)));
```

### Dark/Light Mode

Components support light mode via `.light` class on parent:

```rust
html!("div", {
    .class_signal("light", is_light_mode.signal())
    .child(button!({ .content(Some(text("Themed"))) }))
})
```

Theme classes use `is(.light *)` pseudo-class for light mode variants.

### Theme CSS Variables

- `dwui-bg-primary-{50-950}` - Primary backgrounds
- `dwui-bg-void-{50-950}` - Void/neutral backgrounds
- `dwui-text-on-primary-{50-950}` - Text colors
- `dwui-border-primary-{50-950}` - Border colors
- `dwui-border-error-{50-950}` - Error border colors
