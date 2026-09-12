# dwind-dviz — project plan

A data visualization crate family for the dwind / dwui / dominator ecosystem.
Goals, in priority order:

1. **Correct by construction.** Charts follow established dataviz practice
   without the user having to know it (one axis, fixed categorical hue order,
   thin marks, legends and direct labels, accessible fallbacks).
2. **Reactive to the core.** Everything is a `futures-signals` signal, so the
   same chart renders a static `Vec` or a live stream with no API change.
3. **Extensible.** New marks, scales, and data sources can be added from
   outside the crate. Charts are composed from layers, not picked from an enum.
4. **Themed like dwui.** Reads `--dwui-*` tokens, adds a small set of
   `--dviz-*` tokens, works in light and dark with *selected* dark steps.

Non-goals for 1.0: 3D, geographic maps, WebGL, a grammar-of-graphics DSL.

---

## 1. Architecture

Three crates in one workspace plus a gallery app, all edition 2024. The split keeps all the math
native-testable and lets a second renderer (canvas) reuse the geometry later.

```
dwind-dviz/
  crates/
    dwind-dviz-core/  pure Rust: data model, scales, ticks, layout, geometry, palette
    dwind-dviz-data/  data sources: static, signal-backed, windowed/realtime, downsampling
    dwind-dviz/       dominator renderer: components, marks, axes, legend, tooltip, theme
  examples/
    gallery/        trunk app: every chart, static + live, light/dark; deployed to GH pages
```

### dwind-dviz-core (no DOM, no wasm dependency)

Compiles and tests natively with `cargo test`. Nothing here touches
`web-sys` or `dominator`.

| Module | Contents |
|---|---|
| `data` | `Datum`, `Series<X, Y>`, `Table` (columnar), `Domain` (continuous / ordinal / time) |
| `scale` | `Linear`, `Log`, `Sqrt`, `Time`, `Band`, `Point`, `Ordinal`, `Sequential` (color), `Diverging` (color); the `Scale` trait (`map`, `invert`, `ticks`, `nice`) |
| `ticks` | nice-number tick generation, time tick intervals, tick label formatting |
| `format` | SI / compact numbers, percentages, durations, dates; locale-neutral defaults |
| `geom` | path builders emitting SVG `d` strings: line, area, step, monotone/basis curves, arc, stack layouts, bin/histogram, rounded-end bars |
| `layout` | margins, plot rect, axis label measurement hooks, legend placement, small multiples grid |
| `stats` | extent, quantiles, binning, moving average, LTTB downsampling |
| `palette` | categorical/sequential/diverging/status ramps in OKLab; the six-check validator (lightness band, chroma floor, CVD delta, normal-vision floor, contrast) as a library function *and* a test |

Extension points: `Scale` is a trait; `geom` functions take iterators, not
concrete containers; `palette` accepts user ramps and snaps to passing steps.

### dwind-dviz-data

Bridges plain data and signals. Also DOM-free.

- `DataSource<T>` trait: yields a `SignalVec<T>` (or `Signal<Vec<T>>` for
  whole-replace) plus a `Signal<Domain>` for the axis extent.
- `StaticSource` — wraps a `Vec<T>` (pre-prepared data).
- `MutableSource` — wraps a `MutableVec<T>` the app already owns.
- `WindowedSource` — ring buffer with a retention policy (by count or by
  time span), `push`/`extend`, emits a sliding domain. This is the realtime
  primitive.
- `StreamSource` — adapts any `futures::Stream<Item = T>` into a
  `WindowedSource` (WebSocket, SSE, interval polling all become one call).
- `Downsampled` — wraps a source and applies LTTB / min-max bucketing when the
  point count exceeds the pixel width, driven by a width signal from the
  renderer.
- Realtime controls exposed as signals: `follow: Mutable<bool>` (auto-scroll
  vs. paused), `now: Signal<Instant>` for the trailing edge of a time window.

### dwind-dviz (renderer)

Depends on published `dwind 0.8`, `dwui 0.10`, `dominator 0.5`. Uses the same
`#[component(render_fn = …)]` macro pattern as dwui so props, signals, and
`apply` behave identically.

**Composition model**: a `chart()` container owns the size, the scales, and a
`ChartContext` (scales, plot rect, theme, hover state). Layers are functions
`Fn(&ChartContext) -> Dom` that draw into the plot area. Users add layers;
the crate ships a standard set:

- `axis_x` / `axis_y`, `grid` (recessive by default)
- marks: `line`, `area`, `bars` (grouped / stacked), `points`, `cells`
  (heatmap), `arc` (donut, with the "prefer bars" doc note)
- `legend` (always for >= 2 series, never for 1; toggling is opt-in)
- `labels` (selective direct labels: ends of lines, last bar, extrema)
- `crosshair` + `tooltip` (on by default for line/area; per-mark on bar/dot)
- `annotations` (reference lines, bands, event markers — realtime needs these)
- `live_indicator` (pulsing dot + "live" text when a windowed source is following)

**Rendering strategy**: SVG via `dominator::svg!`. Lines and areas are a single
`<path>` whose `d` is a `String` signal, so a 10k-point live series costs one
attribute update per frame, not 10k nodes. Bars/points use
`children_signal_vec` keyed by series id so incremental updates patch, not
rebuild. Frame updates from live sources are coalesced through
`requestAnimationFrame` (one repaint per frame max).

A canvas mark layer is a later addition, not an abstraction now: because
`geom` produces geometry independently of the DOM, a canvas `line` layer is
a second consumer of the same path data.

**Sizing**: `ResizeObserver` on the container feeds a `Signal<(w, h)>`; scales
are re-ranged reactively. Every chart is responsive with no user code.
Note for implementers: dominator's `class()` panics on a name containing
whitespace; pass an array of names. And dominator's root `DomHandle` leaks removal callbacks
without running them, so anything that must outlive the insertion callback
(the observer and its closure) is kept alive inside the chart's frame
future, never in an `after_removed` closure.

**Higher-level presets** (thin wrappers over the layer API, for the 90% case):
`line_chart`, `bar_chart`, `area_chart`, `scatter_chart`, `sparkline`,
`stat_tile` (headline number + delta + optional sparkline), `heatmap`,
`small_multiples`.

**Theme**: new `--dviz-*` tokens for categorical order (8 slots), sequential
hue, diverging pair + neutral midpoint, status (good/warning/serious/critical),
grid/axis ink, and a texture fill for forced-colors / print. Defaults are
derived from the dwui palette and validated by the `dwind-dviz-core` test suite.
Light and dark are separate, hand-selected steps, not an inversion.

**Accessibility**: `role="img"` + `aria-label` + `<title>`/`<desc>`; a
`table_view` component that renders any `DataSource` as a `dwui` data table;
keyboard focus moves between marks with the tooltip following focus; series
identity is never color alone (legend + direct labels + optional texture).

---

## 2. Data visualization rules baked in

These are enforced by API shape or documented as defaults, not left to users:

- **One y-axis per chart.** No dual-axis API. The docs point to small
  multiples or indexing to a common base.
- **Categorical hue order is fixed by series id**, not by position. Filtering
  out a series never repaints the survivors. A 9th series folds into "Other".
- **Sequential = one hue, light to dark; diverging = two hues + neutral gray
  midpoint.** No rainbow ramps in the crate.
- **Marks are thin**, bars have 4px rounded data-ends anchored to the
  baseline, a 2px surface gap between adjacent/stacked fills, 2px lines,
  >= 8px markers.
- **Text uses text tokens**, never the series color.
- **Status colors are reserved** and ship with an icon + label.
- **Tooltips on by default**, hit targets larger than the mark.
- **Realtime charts don't animate transitions** on data arrival (only on
  discrete user-driven changes), keep a fixed y-domain unless told otherwise
  (to avoid axis jitter), and show a clear live/paused state.

---

## 3. Phases

Each phase ends with the gallery updated and CI green (native tests for
core/data, Chrome + Firefox wasm tests for the renderer, like dwind's
`rust.yml`).

### Phase 0 — Scaffold (small) — done 2026-09-11
- Workspace with the three crates and the gallery; copy dwind's CI, trunk
  config, and `line-tables-only` debug profile.
- Edition 2024 workspace, `rust-version` pinned.
- README with the architecture diagram above; CHANGELOG in dwind's style.

### Phase 1 — Core math (native only) — done 2026-09-11
- `scale` (linear, time, band, log), `ticks`, `format`, `geom` (line, area,
  bars, stack), `stats` (extent, LTTB).
- `palette` with the six-check validator; ship default ramps; validator runs
  as a unit test against both surfaces.
- Property tests for scale round-trips and tick niceness.

### Phase 2 — Rendering foundation — done 2026-09-11
- `chart()` container, `ChartContext`, ResizeObserver sizing, theme tokens
  and `apply_style_sheet` in dwui's style.
- `axis_x`, `axis_y`, `grid`, `line` mark. First static line chart in the
  gallery in light and dark.
- wasm tests: chart mounts, resizes, re-ranges scales, axis ticks match core.

### Phase 3 — Chart forms — done 2026-09-11
- `bars` (grouped, stacked), `area`, `points`, `legend`, `labels`,
  `annotations`.
- Presets: `line_chart`, `bar_chart`, `area_chart`, `scatter_chart`,
  `sparkline`, `stat_tile`.
- Then: `cells` (heatmap), `arc` (donut), `small_multiples`.

### Phase 4 — Interaction — done 2026-09-11
- `crosshair` + `tooltip` (portaled like dwui's popovers), per-mark hover,
  keyboard navigation over marks, legend toggling, brush-to-zoom on time axes.
- Gallery: interaction page.

### Phase 5 — Realtime — done 2026-09-11
- `dwind-dviz-data`: `WindowedSource`, `StreamSource`, `Downsampled`, follow/pause.
- rAF coalescing in the renderer; live indicator; fixed-domain guidance.
- Gallery: a live page fed by an in-browser generator (and a WebSocket demo
  if a small server example is worth it).
- Benchmark: 8 series × 10k points at 60fps in Chrome; record numbers in the
  README.

### Phase 6 — Accessibility, docs, release
- `table_view`, textures for forced-colors, aria audit of every preset.
- rustdoc with an example per component; gallery deployed to GH pages.
- Publish `dwind-dviz-core`, `dwind-dviz-data`, `dwind-dviz` 0.1.0.

Deferred from Phase 5: `x_extent` and `downsample` on `area_chart!`, a
WebSocket demo (needs a server example), and a paint-inclusive benchmark
(the recorded figure is the synchronous cost; browser paint is not
measurable from wasm-bindgen-test).

Note for implementers: `dwind_macros::dwkeyframes!` with `register_fn`
panicked when invoked from this crate's stylesheet installer; at-rules
(keyframes, media queries) are injected as a plain `<style>` element in
`theme::inject_at_rules` instead.

Deferred from Phase 4: touch-specific affordances (a long-press to pin the
tooltip), a Voronoi/nearest-point layer for dense scatter, and pinning a
tooltip on click.

Deferred from Phase 3: horizontal bars (needs a band y with a linear x),
leader lines for converging end labels, and in-segment labels for stacked
bars (needs DOM text measurement; until then totals sit on the caps).

Later (post-1.0 candidates): canvas mark layer for very large series,
candlestick / OHLC, box plot, streamgraph, export to PNG/SVG, an
`egui`/Tauri-friendly headless render path.

---

## 4. Decisions (settled 2026-09-11)

1. **Crate names.** `dwind-dviz-core`, `dwind-dviz-data`, `dwind-dviz`.
   Library paths are `dwind_dviz_core::…` etc.; the gallery is
   `examples/gallery`.
2. **Separate repo** from dwind, depending on published `dwind 0.8` and
   `dwui 0.10`.
3. **Edition 2024.** MSRV follows whatever the edition requires (1.85+);
   pin it in `rust-version` and check it in CI.
4. **Time type.** jiff is a plain dependency of `dwind-dviz-core`, no
   feature flags and no chrono/time support. Time axes take `jiff::Timestamp`
   and tick labelling uses jiff's calendar and timezone logic; the `js`
   feature is enabled so it works on `wasm32-unknown-unknown`. Apps on chrono
   convert at the boundary with `timestamp_millis()`; that is a one-liner and
   not our problem to maintain.
5. **SVG only for 1.0.** Canvas is deferred until a real workload exceeds SVG.
