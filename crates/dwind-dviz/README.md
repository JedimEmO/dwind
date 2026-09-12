# dwind-dviz

Reactive, accessible data visualization for the [dwind](https://github.com/JedimEmO/dwind)
/ dwui / [DOMINATOR](https://github.com/Pauan/rust-dominator) ecosystem.
Renders pre-prepared data and live streams through the same API.

For exact values and screen-reader users, pair a chart with `table_view`:

```rust
html!("section", {
    .child(line_chart!({ .label("Latency").series_signal(source.series_signal()) }))
    .child(table_view(source.series_signal()))
})
```

Use `point_table_view` when every plotted observation must be exposed rather
than a latest/minimum/maximum summary. Custom x/y scales implement
`CustomScale`, and live sources that expose reactive extents implement
`DomainSource`.

| Crate | What it is |
|---|---|
| `dwind-dviz-core` | DOM-free math: scales, ticks, formatting, SVG geometry, layout, stats, validated palettes. Tested natively. |
| `dwind-dviz-data` | Signal-backed data sources: static, mutable, windowed/realtime, stream adapter, downsampling. |
| `dwind-dviz` | The SVG renderer: chart container, layers (axes, marks, legend, tooltip, annotations), presets, theme. |

The gallery lives in the dwind example site: `examples/webpage`, route `#/charts`,
deployed at https://jedimemo.github.io/dwind/examples/#/charts. It shows every
chart, static and live, on the dark surface and the light one.

Status: **Core, renderer, interaction, realtime, and API hardening foundations are implemented; release verification is next**. See [PLAN.md](PLAN.md) for the architecture,
the dataviz rules the crate enforces, and the phase roadmap.

## Realtime

Feed a `WindowedSource`, call `commit_on_frame` once, and pass its signals
to a preset:

```rust
let source = WindowedSource::new(Retention::Span(60_000.0)); // 60 s of history
commit_on_frame(&source);                                    // one publish per frame
source.push("api", Point::new(now_ms, value));               // from a timer, stream, socket

line_chart!({
    .x(XKind::time_utc())
    .x_extent_signal(source.window_signal().map(Some))       // sliding window
    .y(Some(Extent::new(0.0, 220.0)))                        // fixed: no axis jitter
    .downsample(true)                                        // min/max to plot width
    .series_signal(source.series_signal())
})
```

Benchmark (Firefox 2026-09, headless, `tests/live.rs`): eight series of
10,000 points each, downsampled to an 800px plot, steady-state push of one
sample per series per frame.

| Measure | Value |
|---|---|
| Synchronous cost per frame (commit + downsample + path patch) | 1.3 ms |

That leaves the rest of the 16 ms frame budget to the browser's own layout
and paint of eight `<path>` elements of at most 1,600 points each.

## Development

From the dwind workspace root:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli   # the version must match Cargo.lock
cargo install trunk

cargo test -p dwind-dviz-core -p dwind-dviz-data                              # native tests
CHROMEDRIVER=$(which chromedriver) cargo test -p dwind-dviz --target wasm32-unknown-unknown  # browser tests
cd examples/webpage && trunk serve --open                                     # site on :8811, charts at #/charts
```
