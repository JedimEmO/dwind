# DWIND webpage

The dwind documentation site — and the largest worked example of dwind + dwui in
the repository. Everything on it is compiled Rust running as WebAssembly; there
is no JavaScript beyond the wasm-bindgen glue and no CSS build pipeline.

## Running the webpage in local developer mode

You need a recent (>= 1.79.0) version of rust, please refer to https://rustup.rs/ for how to install this.

After installing those, add the `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```

To run the application

### Trunk

First install the trunk utility: https://trunkrs.dev/guide/getting-started/installation.html

then do

```shell
trunk serve --open
```

> Trunk does not watch the sibling workspace crates. After editing `crates/dwind`
> or `crates/dwui`, restart `trunk serve` to pick up the rebuild.

## How the site is put together

| Module | What lives there |
| --- | --- |
| `lib.rs` | App shell: router, sticky header with active-route indicator, scroll-progress rail, docs shell and prev/next pager, footer |
| `fx.rs` | Reusable reactive effects — pointer spotlight, 3D tilt, magnetic controls, aurora background, film grain, scroll-spy, marquee, kinetic headlines, glass and scrollbar surfaces |
| `keyframes.rs` | Every animation on the site, declared with `dwkeyframes!` |
| `palette.rs` | The ⌘K command palette: fuzzy search over every route, full keyboard control |
| `reveal.rs` | `IntersectionObserver`-driven progressive reveal on scroll |
| `pages/signal_lab.rs` | The reactivity demo on the home page |
| `pages/docs/` | Documentation pages, sidebar, live example frames, syntax-highlighted source |

### There is no app stylesheet

This example used to carry ~300 lines of hand-written CSS. It now carries none.
The only `<style>` block left is in `public/index.html`, holding exactly the
things that are document-level rather than component-level: the font-family
classes, `:root` colour-scheme, `::selection`, a site-wide `:focus-visible` ring,
and the blanket `prefers-reduced-motion` clamp.

Everything else went back into dwind:

- **Keyframes** are `dwkeyframes!` declarations in `keyframes.rs`, which mint
  compile-checked `animate-*` classes and inject their rule lazily.
- **The pointer spotlight** — a `::before` glow and a masked `::after` ring, both
  tracking the cursor — is `dwclass!` variants. `content: ""` is supplied
  automatically, and the mask declarations that have no utility use the
  `[property:value]` escape hatch.
- **The scroll-reveal cascade**, including the parent-state `.reveal-in > *`
  rules and the `:nth-child` stagger, is child-selector variants.
- **Scrollbars, glass panels and the marquee hover-pause** are variants too —
  `[&::-webkit-scrollbar-thumb]:`, `[&:hover > *]:`.

### The effects are signals, not an animation library

Nothing here reaches for a JS animation runtime. Each effect is a `Mutable` that
an event writes and a `style_signal` reads, so a pointer move updates exactly the
CSS properties that depend on it — no `requestAnimationFrame` loop, no re-render,
no diff:

```rust
// fx.rs — the whole spotlight, minus bookkeeping
.event(move |e: events::PointerMove| {
    pos.set_neq(normalised(&element, e.mouse_x() as f64, e.mouse_y() as f64));
})
.style_signal("--sx", pos.signal().map(|(x, _)| format!("{:.2}%", x * 100.0)))
```

The home page's reactivity lab makes this measurable: it counts every property
write as it happens, next to a re-render count that stays at zero.

### Keyboard

- <kbd>⌘K</kbd> / <kbd>Ctrl</kbd>+<kbd>K</kbd> or <kbd>/</kbd> — open the command palette
- <kbd>↑</kbd> <kbd>↓</kbd> — move through results, <kbd>⏎</kbd> to open, <kbd>Esc</kbd> to dismiss

Everything interactive is reachable by keyboard, and `prefers-reduced-motion` is
honoured throughout — the marquee, sheen, tilt, and reveal animations all stop.
