# dwind changelog

## dwind 0.8.0 / dwind-macros 0.5.0 / dwind-base 0.1.2 / dwui 0.9.1 - 2026-07-25

Everything here is additive. The theme: raw CSS in an application was almost
always needed because of a *selector* or an *at-rule*, never because of a
property. These three additions close that gap.

### `dwkeyframes!` — declare animations without a raw stylesheet

Utility classes are single declaration blocks, so a `@keyframes` could never be
one; the CSS binding pipeline has no at-rule support at all. Every crate worked
around it with a hand-written `&str` blob pushed through `stylesheet_raw` — no
deduplication, no collision detection, and every keyframe paying its cost whether
the page used it or not.

```rust
dwkeyframes! {
    #[animation("900ms cubic-bezier(0.16, 1, 0.3, 1) both")]
    fade_up {
        "from" => "opacity: 0; transform: translateY(14px);",
        "to"   => "opacity: 1; transform: translateY(0);",
    }
}

html!("div", { .dwclass!("animate-fade-up") })
```

- Emits the at-rule *and* a compile-time-checked `animate-*` utility, so a typo
  in the class name is still a build error.
- The rule is injected the first time the class — or the handle's `Display` —
  is used, and never twice. `format!("{FADE_UP_KEYFRAMES} 600ms {delay}ms")`
  registers as a side effect, for shorthands composed at runtime.
- Names are namespaced by the consuming crate by default (`#![prefix = "..."]`
  to override, `#[name = "..."]` to pin an exact name). Registering one name with
  two different bodies now panics in debug builds instead of silently winning.
- New `dwind::prelude::keyframes` module (`Keyframes`, `register`,
  `is_registered`, `registered_css`) backs it.
- dwind's own `spin`/`ping`/`pulse`/`bounce` and dwui's five `dwui-*` keyframes
  now use it. Names and injection timing are unchanged;
  `append_animation_keyframe_style()` keeps its signature, and
  `dwui::theme::apply_style_sheet` is now idempotent.

### Arbitrary declarations — `[property:value]`

The escape hatch for properties with no utility, matching Tailwind:

```rust
.dwclass!("[mask-composite:exclude] [--sx:50%] hover:[color:red]")
```

Underscores in the value become spaces (`[transition:opacity_650ms_ease]`), since
a class string is space-separated. The property side is left alone so custom
properties keep their underscores.

Unambiguous against the variant syntax because a variant's `]` is always followed
by `:`. Previously a bracket group *without* a trailing colon failed every parser
and was silently discarded **along with every class after it** — so
`dwclass!("foo [a:b] bar")` yielded one class, not three. That truncation is
fixed, and a bracket group with no colon at all is now a compile error with a
message instead of silence.

### Pseudo-elements that actually render

`[&::before]:` variants already parsed, but a `::before` with no `content` never
generates a box, so the utility did nothing on its own. dwind now emits
`content: ""` for any variant whose last compound targets `::before`/`::after`.
Because `DomBuilder::raw` appends rather than replaces, your own `content-[...]`
later in the same class still wins — no `--tw-content` indirection needed.

Added shorthands: `before:`, `after:`, `placeholder:`, `marker:`, `selection:`,
`backdrop:`, `first-letter:`, `first-line:`.

**Behaviour change:** `before:`/`after:` previously rendered as the legacy
single-colon `:before`. They now render `::before` (equivalent in every engine)
*and* gain generated content. If you wrote `before:` and supplied `content`
elsewhere, check it. This is the only currently-working input whose meaning
changes.

**Bug fix:** `render_generator` ignored the bracketed variant entirely, so
`[&::before]:bg-color-[red]` compiled and silently styled the element itself.
Generators now honour variants.

### New utilities

`delay-0`…`delay-1000`, `underline` / `overline` / `line-through` /
`no-underline`, `tracking-tighter`…`tracking-widest`, `whitespace-*`,
`list-none` / `list-disc` / `list-decimal`, `content-empty` / `content-none`,
`font-inherit`, `isolate` / `isolation-auto`, `mix-blend-*`, `will-change-*`,
`outline-none` / `outline-hidden`.

### Tests

`dwind-macros` had no tests at all; it now has 29 covering the grammar and, for
the first time, codegen. New browser suite at `crates/dwui/tests/styling.rs`
asserts lazy injection, deduplication, that `::before` materialises, and that
arbitrary declarations reach `getComputedStyle`.

## dwui 0.9.0 - 2026-06-10

### Heavy components

- `data_table!` — themed table with sortable headers (`aria-sort`, controlled `on_sort`), rich `Dom` cells, and optional clickable rows
- `virtual_scroll!` — windowed list rendering only visible rows (tested against 50k items), `aria-setsize`/`aria-posinset`, and `on_reach_end` infinite loading with an `aria-busy` spinner row
- `date_picker!` — calendar popup (`role="dialog"` + `role="grid"`) with month navigation, full keyboard support (arrows, PageUp/PageDown, Home/End, Escape), outside-click close, and a controlled `CalendarDate` value
- `CalendarDate` — dependency-free civil date type (ISO-8601 parse/format, date arithmetic) backing the picker

### New components

- `alert!` — status callouts (info/success/warning/error/neutral) with severity-aware ARIA roles and an optional dismiss button
- `accordion!` — animated disclosure panels with `aria-expanded`/`aria-controls` and region landmarks
- `tooltip!` — hover/focus tooltips with `role="tooltip"` and `aria-describedby` linkage, four placements
- `avatar!` — image avatars with initials fallback in three sizes
- `divider!` — labelled separator (`role="separator"`)
- `breadcrumbs!` — `nav aria-label="Breadcrumb"` trails with `aria-current="page"`
- `skeleton!` — pulsing loading placeholders (text/circle/rect)
- `spinner!` — `role="status"` loading indicator in three sizes

### Theme

- New `success` and `warning` color variable families with bg/border/text utility classes; `ColorsCssVariables::with_status_colors` for custom palettes (defaults: apple/candlelight)
- New `dwui-text-primary-*` utility classes
- `pretty_list!` is now fully theme-driven (no hard-coded palette colors)

### Buttons

- `ButtonSize::{Small, Medium, Large}` prop
- New `ButtonType::Text` low-emphasis variant

### Example webpage

- New dwind utility showcase at `#/examples` — eight artifacts (responsive layout, gradients, type scale, glassmorphism, motion, bracket variants, composition, custom generators) built purely from dwind classes, each with browser-viewable source
- Every dwui gallery card now ships a "view source" toggle with compile-time highlighted Rust
- The dwind showcase reveals progressively on scroll (shared IntersectionObserver + staggered CSS cascade, disabled under prefers-reduced-motion)
- New "Getting started" docs page introducing the two layers separately; the dwind docs no longer demo with dwui components
- Complete redesign: landing page with hero, bento feature grid, and live component previews; full component gallery at `#/components`; restyled docs layout — Bricolage Grotesque / IBM Plex Sans / JetBrains Mono typography with a candlelight-on-woodsmoke industrial theme

## dwui 0.8.0 - 2026-06-10

### New components

- `switch!` — toggle switch with `role="switch"`, `aria-checked`, and an associated clickable label
- `checkbox!` — checkbox with `role="checkbox"`, animated checkmark, and label association
- `progress!` — progress bar with `aria-valuenow/min/max` and an indeterminate mode
- `badge!` — status pill with primary/error/void/outline variants
- `tab_list!` — WAI-ARIA tabs with roving tabindex and arrow-key/Home/End navigation

### Accessibility

- Labels are now associated with their inputs (`label[for]` ↔ `input[id]`) in `text_input!`, `select!`, and `slider!`
- Validation errors are announced via `role="alert"` and linked with `aria-describedby`/`aria-invalid`
- `modal!` now has `role="dialog"`, `aria-modal`, a configurable `aria_label`, focus management, and a labelled close button
- `button!` renders `type="button"` and visible keyboard focus rings (`:focus-visible`)
- `heading!` supports semantic levels (`HeadingLevel::H1`–`H6`)
- `pretty_list!` items are keyboard operable (Tab + Enter/Space) and expose `aria-current`
- New `dwui-ring-primary-*` / `dwui-ring-error-*` utility classes for themed focus rings

### Visual polish

- Focus rings, active/hover states, and smooth transitions across all interactive components
- Modal entrance animation with backdrop blur
- Cards have borders, shadows, and rounded-lg corners

### Testing

- Added native unit tests for input validation
- Added a browser test suite (wasm-bindgen-test) covering rendering, ARIA semantics, and interactions for all components: `wasm-pack test --headless --firefox crates/dwui`

## 0.2.0 - 2024-12-19

- Added new rounded classes
