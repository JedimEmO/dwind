# dwind changelog

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
