# dwind-dviz changelog

History from before the crates moved into the dwind workspace (September 2026).
Later changes are recorded in the workspace CHANGELOG at the repository root.


## Unreleased

- Gallery restyled to match the dwind site: woodsmoke ground, candlelight
  accent, glass preview frames, aurora and grain, Bricolage Grotesque /
  IBM Plex Sans / JetBrains Mono, scroll reveal and a sticky chapter rail.
  Light mode stays available (`?light` or the header toggle) for
  screenshots and as the light-surface test bed.
- Chrome neutrals moved to the woodsmoke scale (cool grays) in both modes,
  so charts sit on a dwui surface without a warm/cool mismatch. Every
  status and categorical contrast still clears its threshold; the two
  exact-number reference assertions were updated.
- New host hooks: `--dviz-accent` (focus rings, brush, control hovers;
  defaults to series slot 1), `--dviz-font-display` (stat and donut
  figures) and `--dviz-font-mono` (axis ticks, value labels, tooltips,
  legend numbers, small controls). Both font tokens fall back to the
  inherited body face when unset.

- Keyed lists are diffed (`keyed::diffed`), not replaced: hiding or showing
  a series now removes or inserts only that series' nodes. Survivors keep
  their DOM, their state and their in-flight tweens, and no longer replay
  entrance animations. Applies to every mark layer, ticks and gridlines,
  the legend, the donut's value legend, and the crosshair markers.

- Donut redesign: rounded stroked segments on one ring with a centre
  figure (the total, or the hovered segment's value and share), hover lift,
  a value legend with share percentages that toggles and highlights
  segments, and a sweep-in entrance. Segment angles are tweened in Rust
  (`motion::tween`), since the browser's path interpolation cannot cross
  the large-arc flag cleanly.
- Jank fixes: the tooltip is one persistent element (its entrance no longer
  restarts on every pointer move); crosshair markers are persistent nodes;
  the line draw-in selector works again; live charts drop the glow filter
  and entrance animations; the line preset no longer rebuilds its line
  layer when the series count crosses one (`Wash::WhenSingle` decides per
  render instead).

- Motion model. Marks are persistent keyed nodes fed by geometry signals:
  bars (by category), points (by index), cells (by grid position), arcs,
  areas and lines (by series) patch attributes in place, so under the vivid
  style a data change tweens (`d`, `cx`/`cy`, `x`/`y`/`width`/`height`,
  `fill`) instead of jumping. Removed series and marks linger for 260ms
  with `dviz-leave` and fade (bars sink to the baseline); new ones get
  `dviz-enter`. Axis ticks and gridlines are keyed by label and slide to
  new positions on zoom or domain change, fading in and out. Tooltips are
  signals: a hovered mark that changes shows the new value, and a mark
  removed while hovered clears the tooltip. `chart!`'s `live` (set by
  `line_chart!`'s `live`) turns every tween off so a streaming chart never
  lags its data.

- Vivid style (default on; `theme::set_vivid(false)` or `.vivid(false)` on
  `chart!` for the quiet look): vertical fade gradients under washes,
  single areas and sparklines; a soft glow on lines and crosshair markers
  on the dark surface; draw-in, grow, pop and fade entrance motion on a
  series' first render only; a faint rounded plot surface; a glassy tooltip
  that rises in; pill legend items; a pulsing halo on the newest point of
  a live line (`.live(true)`); stat tile values count up to a new figure.
  Reference labels sit at the left of the plot, clear of end markers. All
  motion is disabled under `prefers-reduced-motion`.

- Visual and bugfix pass: marks are clipped to the plot through a per-chart
  `<clipPath>` (zooming no longer draws lines past the axes); end labels and
  the crosshair ignore points outside the visible x range; lines get an
  8px end marker with a surface ring and, on the line preset, a faint wash
  under a single series; hovering a legend item dims the other series;
  bars, cells, slices and points brighten on hover; the tooltip fades in.
  All motion respects `prefers-reduced-motion`.

- Phase 5: realtime. `dwind-dviz-data` ships `WindowedSource` (per-series
  ring buffers, `Retention::Count` or `Retention::Span`, batched `commit`,
  a notifier hook, a sliding window signal with follow/pause), `drive` for
  feeding any `Stream` of samples, and `downsample` (min/max buckets or
  LTTB to the plot width). The renderer adds `commit_on_frame` (one publish
  per animation frame), `live_indicator` (Live/Paused toggle), the
  `width_tap` layer, and `x_extent` / `downsample` props on `line_chart!`.
  Gallery gained a live section with eight series at up to 60 Hz and a
  10k-sample burst. Benchmark below.

- Phase 4: interaction. `ChartContext` gains tooltip and hover-x state; the
  container renders the tooltip (flipped away from the edges, `aria-live`).
  `crosshair` layer: snaps to the nearest sample across series, lists every
  series in the tooltip, ring markers on the lines, a focusable overlay with
  arrow/Home/End/Escape navigation, and an optional brush that reports an x
  range. Bars, points (24px hit target), cells and arcs show per-mark
  tooltips on hover and keyboard focus. `SeriesVisibility` plus
  `legend_with`: legend items are buttons with `aria-pressed` that hide a
  series while survivors keep their slot. Presets: `toggle_legend` (on by
  default) and, on line and area charts, `zoomable` with a "Reset zoom"
  control. Gallery gained an interaction section.

- Phase 3: chart forms. Layers `bars` (grouped, stacked, 100% stacked;
  24px cap, rounded data end, 2px surface gaps), `area` (wash or stacked
  bands), `points` (8px markers with a surface ring), `line_end_labels`,
  `bar_value_labels` (per bar, or column totals when stacked; dropped past
  16 bars), `reference_y`, `reference_x`, `band_y`, `cells` (heatmap on two
  band axes, sequential or diverging, mode-aware) and `arcs` (donut).
  `SeriesSlots` is shareable between a chart and its HTML `legend` (hidden
  for one series). Presets `line_chart!`, `area_chart!`, `bar_chart!`,
  `scatter_chart!`, `sparkline!`, `stat_tile!`, `donut_chart!` (folds past
  six segments) and `small_multiples`. `YDomain::Band` for heatmaps;
  `theme::set_mode` for Rust-side color scales; `extra_right` on `chart!`
  so end labels fit. Gallery grew to tiles, lines and areas, bars and
  distributions sections.

- Phase 2: rendering foundation in `dwind-dviz`. `chart!` container with a
  `ResizeObserver`-driven `Frame` (plot rect plus resolved x/y scales:
  linear, log, time, band), automatic axis margins, `role="img"` with
  `aria-label` and `<title>`; `--dviz-*` theme tokens for dark (`:root`) and
  light (`.light`) from the validated palette; `axis_x`, `axis_y`, `grid`
  and `line` layers, the line being one `<path>` per series keyed by id with
  a signal-driven `d`, so survivors keep their color slot when a series is
  removed. Browser tests cover mounting, tick agreement with core, keyed
  paths, resize re-ranging, and gaps. Gallery shows two line charts with a
  light/dark toggle (`?light` on load).

- Phase 1: `dwind-dviz-core` math. `data` (extents, points, series),
  `scale` (linear, log, pow/sqrt, time, band/point, sequential and diverging
  color), `ticks` (1/2/5 linear steps, log decades, calendar-aware time
  ticks with DST-safe local snapping via jiff), `format` (grouped, SI,
  compact, percent, signed, duration, multi-scale time labels), `geom`
  (path builder, line/area/band with step and monotone curves, rounded-end
  bars, annular arcs and pie layout, stacking with expand and diverging
  offsets), `layout` (rects, margins, small-multiples grid), `stats`
  (extent, quantiles, bins, moving average, LTTB and min/max downsampling),
  and `palette` (sRGB/OKLab, Machado CVD simulation, ramps, the categorical
  and ordinal validators, and the validated default palette). 66 native
  tests, including a check that the shipped palette reproduces the reference
  ΔE figures in both modes.

- Phase 0: workspace scaffold with `dwind-dviz-core`, `dwind-dviz-data`,
  `dwind-dviz`, and the gallery app; CI for native and browser tests; Pages
  deployment for the gallery and docs.
