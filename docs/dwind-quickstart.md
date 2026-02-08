# DWIND Macro Quick Reference

## dwclass! - Basic Usage

Apply utility classes to DOMINATOR elements:

```rust
html!("div", {
    .dwclass!("flex gap-4 p-4")           // Multiple classes
    .dwclass!("bg-gray-900 text-white")   // Chain multiple calls
})
```

With `.apply()` for conditional logic:

```rust
html!("div", {
    .apply(|b| {
        match variant {
            0 => dwclass!(b, "bg-red-500"),
            1 => dwclass!(b, "bg-blue-500"),
            _ => dwclass!(b, "bg-gray-500"),
        }
    })
})
```

## dwclass_signal! - Reactive Styling

Toggle classes based on signals:

```rust
let is_active = Mutable::new(false);

html!("div", {
    .dwclass!("p-4 rounded")
    .dwclass_signal!("bg-blue-500", is_active.signal())      // Applied when true
    .dwclass_signal!("opacity-50", not(is_active.signal()))  // Applied when false
})
```

## Responsive Breakpoints

Prefix classes with `@breakpoint:` to apply at specific screen sizes:

| Prefix | Width | Description |
|--------|-------|-------------|
| `@xs:` | < 640px | Default (no prefix needed) |
| `@sm:` | >= 640px | Small screens |
| `@md:` | >= 1280px | Medium screens |
| `@lg:` | >= 1920px | Large screens |
| `@xl:` | >= 2560px | Extra large screens |
| `@<sm:` | < 640px | Less than small |

```rust
.dwclass!("flex-col @sm:flex-row")           // Column on mobile, row on sm+
.dwclass!("gap-2 @md:gap-4 @lg:gap-8")       // Increasing gap at breakpoints
.dwclass!("@<sm:hidden @sm:block")           // Hidden on mobile, visible on sm+
```

Custom media queries:

```rust
.dwclass!("@((max-width: 700px)):bg-red-500")  // Custom breakpoint
```

## Pseudo-Classes

Prefix classes with pseudo-class name:

```rust
.dwclass!("hover:bg-blue-600")               // On hover
.dwclass!("focus:ring-2 focus:ring-blue-400") // On focus
.dwclass!("active:scale-95")                 // On active/pressed
.dwclass!("disabled:opacity-50")             // When disabled
```

Complex pseudo-classes:

```rust
.dwclass!("nth-child(2):bg-gray-800")        // Second child
.dwclass!("nth-child(odd):bg-gray-900")      // Odd children
.dwclass!("is(.selected):font-bold")         // When has .selected class
```

## Variant Selectors

Apply styles to child elements using `[selector]:class` syntax:

```rust
// Style direct children
.dwclass!("[& > *]:p-2")                     // All direct children get p-2
.dwclass!("[> span]:text-blue-500")          // Direct span children

// Complex child selectors
.dwclass!("[& > *]:nth-child(2):bg-red-500") // Second direct child
.dwclass!("[& *]:w-full")                    // All descendants
.dwclass!("[& > div:hover]:bg-gray-800")     // Direct div children on hover
```

Combining variant + pseudo-class:

```rust
.dwclass!("[& > button]:hover:bg-blue-600")  // Direct button children on hover
```

## dwgenerate! - Custom Classes

Pre-declare reusable class combinations:

```rust
// Define custom class
dwgenerate!("btn-primary", "hover:bg-blue-600 active:scale-95");

// Use it
html!("button", {
    .dwclass!("btn-primary px-4 py-2 bg-blue-500")
})
```

With parameters:

```rust
dwgenerate!("custom-bg", "bg-[#ff5500]");    // Custom color
.dwclass!("padding-[20px]")                  // Arbitrary value
```

## Color Opacity

Apply opacity to colors with `/opacity`:

```rust
.dwclass!("bg-blue-500/50")    // 50% opacity background
.dwclass!("text-white/75")     // 75% opacity text
```

## Common Patterns

### Centered Container
```rust
.dwclass!("flex justify-center align-items-center h-full")
```

### Card Layout
```rust
.dwclass!("p-4 bg-gray-900 rounded-lg shadow-lg border border-gray-800")
```

### Responsive Grid
```rust
.dwclass!("grid grid-cols-1 @sm:grid-cols-2 @md:grid-cols-3 gap-4")
```

### Button with States
```rust
.dwclass!("px-4 py-2 bg-blue-500 rounded")
.dwclass!("hover:bg-blue-600 active:scale-95")
.dwclass!("disabled:opacity-50 disabled:cursor-not-allowed")
```

### Animated Element
```rust
let spinning = Mutable::new(false);
html!("div", {
    .dwclass!("w-8 h-8 bg-blue-500")
    .dwclass_signal!("animate-spin", spinning.signal())
})
```

### Theme-Aware Styling
```rust
// Parent has "light" class for light mode
.dwclass!("bg-gray-900 is(.light):bg-gray-100")
.dwclass!("text-white is(.light):text-gray-900")
```
