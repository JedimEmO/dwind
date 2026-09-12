//! Chart theme tokens (`--dviz-*`), layered on dwui's `--dwui-*` tokens.
//!
//! Dark is the default on `:root`; a `.light` ancestor (the same switch dwui
//! uses) swaps in the light steps. Both sets come from the validated
//! defaults in [`dwind_dviz_core::palette::defaults`]; they are separate
//! hand-selected steps, not an inversion.
//!
//! Marks read their colors through these variables, so re-branding a chart
//! is a matter of overriding `--dviz-series-1`..`8` and the chrome tokens on
//! any ancestor. Keep the *order* of the series slots: it is the
//! colorblind-safety mechanism.
//!
//! Three tokens have no default and let the host pull charts into its own
//! look without touching data colors:
//!
//! - `--dviz-accent`: focus rings, the brush, control hovers. Defaults to
//!   series slot 1; set it to the host's accent (e.g. dwind's candlelight).
//! - `--dviz-font-display`: the big figure in a stat tile and donut center.
//! - `--dviz-font-mono`: axis ticks, value labels, tooltips, legend numbers
//!   and the small controls. Falls back to the inherited body face.

use std::fmt::Write;
use std::sync::Once;

use dominator::stylesheet;
use dwind_dviz_core::palette::{Mode, defaults};
use futures_signals::signal::{Mutable, Signal};

thread_local! {
    static MODE: Mutable<Mode> = Mutable::new(Mode::Dark);
    static VIVID: Mutable<bool> = Mutable::new(true);
}

/// Whether new charts default to the vivid style: gradient washes, a soft
/// glow on lines in dark mode, draw-in and grow animations on first render,
/// a subtle plot surface. Off is the quiet, print-like look. Either way the
/// marks, colors and geometry are the same; only the dressing changes.
pub fn vivid() -> bool {
    VIVID.with(|v| v.get())
}

pub fn set_vivid(vivid: bool) {
    VIVID.with(|v| v.set_neq(vivid));
}

/// The mode Rust-side color scales (sequential, diverging) render for.
/// CSS-driven colors follow the `.light` class on their own; call
/// [`set_mode`] wherever that class is toggled so computed colors agree.
pub fn mode() -> Mode {
    MODE.with(|m| m.get())
}

pub fn set_mode(mode: Mode) {
    MODE.with(|m| m.set_neq(mode));
}

pub fn mode_signal() -> impl Signal<Item = Mode> {
    MODE.with(|m| m.signal())
}

/// Rules dominator's `stylesheet!` cannot express (at-rules), injected once
/// as a `<style>` element.
const AT_RULES: &str = "\
@keyframes dviz-pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.35; } }\
@keyframes dviz-fade-in { from { opacity: 0; } to { opacity: 1; } }\
@keyframes dviz-draw { from { stroke-dashoffset: 1; } to { stroke-dashoffset: 0; } }\
@keyframes dviz-grow { from { transform: scaleY(0.001); } to { transform: scaleY(1); } }\
@keyframes dviz-pop { from { transform: scale(0.001); } to { transform: scale(1); } }\
@keyframes dviz-halo { 0% { r: 4px; opacity: 0.55; } 100% { r: 14px; opacity: 0; } }\
@keyframes dviz-rise { from { opacity: 0; transform: translateY(4px); } to { opacity: 1; transform: translateY(0); } }\
@media (prefers-reduced-motion: reduce) { .dviz-chart *, .dviz-live-dot, .dviz-tooltip, .dviz-stat-value { animation: none !important; transition: none !important; } }\
@media (forced-colors: active) { .dviz-chart .dviz-bar, .dviz-chart .dviz-cell, .dviz-chart .dviz-arc { fill: CanvasText !important; stroke: Canvas !important; stroke-width: 1px; } .dviz-chart .dviz-line { stroke: CanvasText !important; filter: none !important; } .dviz-chart .dviz-end-marker, .dviz-chart .dviz-point { fill: CanvasText !important; stroke: Canvas !important; } .dviz-chart .dviz-grid line { stroke: GrayText !important; } }";

fn inject_at_rules() {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(style) = document.create_element("style") else {
        return;
    };
    style.set_attribute("data-dviz", "at-rules").ok();
    style.set_text_content(Some(AT_RULES));
    if let Some(head) = document.head() {
        head.append_child(&style).ok();
    }
}

/// Number of categorical series slots. A ninth series folds into "Other"
/// and is drawn in the muted ink.
pub const SERIES_SLOTS: usize = 8;

/// The CSS variable that colors categorical slot `slot` (0-based).
pub fn series_var(slot: usize) -> String {
    if slot < SERIES_SLOTS {
        format!("var(--dviz-series-{})", slot + 1)
    } else {
        "var(--dviz-ink-muted)".to_string()
    }
}

/// The raw `--dviz-*` declarations for a mode, without a selector.
pub fn declarations(mode: Mode) -> String {
    let chrome = defaults::chrome(mode);
    let mut out = String::new();
    for (i, c) in defaults::categorical(mode).iter().enumerate() {
        let _ = write!(out, "--dviz-series-{}: {c};", i + 1);
    }
    let _ = write!(
        out,
        "--dviz-accent: var(--dviz-series-1);--dviz-surface: {};--dviz-ink: {};--dviz-ink-2: {};--dviz-ink-muted: {};--dviz-grid: {};--dviz-axis: {};--dviz-delta-up-good: {};",
        chrome.surface,
        chrome.ink_primary,
        chrome.ink_secondary,
        chrome.ink_muted,
        chrome.grid,
        chrome.axis,
        chrome.delta_up_good,
    );
    let _ = write!(
        out,
        "--dviz-status-good: {};--dviz-status-warning: {};--dviz-status-serious: {};--dviz-status-critical: {};",
        defaults::STATUS_GOOD,
        defaults::STATUS_WARNING,
        defaults::STATUS_SERIOUS,
        defaults::STATUS_CRITICAL,
    );
    out
}

/// Installs the dwind-dviz stylesheet. Idempotent; call it once after
/// `dwind::stylesheet()` and `dwui::theme::apply_style_sheet(..)`.
pub fn apply_style_sheet() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        inject_at_rules();
        stylesheet!(":root", {
            .raw(declarations(Mode::Dark))
        });
        stylesheet!(".light", {
            .raw(declarations(Mode::Light))
        });
        // Axis text: muted ink, tabular figures so ticks align.
        stylesheet!(".dviz-axis text", {
            .raw("fill: var(--dviz-ink-muted); font-family: var(--dviz-font-mono, inherit); font-size: 11px; font-variant-numeric: tabular-nums; letter-spacing: 0.01em; user-select: none;")
        });
        stylesheet!(".dviz-axis line", {
            .raw("stroke: var(--dviz-axis); shape-rendering: crispEdges;")
        });
        stylesheet!(".dviz-grid line", {
            .raw("stroke: var(--dviz-grid); shape-rendering: crispEdges;")
        });
        stylesheet!(".dviz-chart", {
            .raw("display: block; position: relative; width: 100%; min-width: 0;")
        });
        stylesheet!(".dviz-chart svg", {
            .raw("display: block; overflow: visible;")
        });
        stylesheet!(".dviz-label", {
            .raw("fill: var(--dviz-ink-2); font-family: var(--dviz-font-mono, inherit); font-size: 11px; font-variant-numeric: tabular-nums; user-select: none; pointer-events: none;")
        });
        stylesheet!(".dviz-reference", {
            .raw("stroke: var(--dviz-ink-muted); stroke-width: 1; stroke-dasharray: 4 3;")
        });
        stylesheet!(".dviz-band", {
            .raw("fill: var(--dviz-ink-muted); fill-opacity: 0.1;")
        });
        stylesheet!(".dviz-sparkline", {
            .raw("fill: none; stroke: var(--dviz-ink-muted); stroke-width: 1.5; stroke-linejoin: round; stroke-linecap: round;")
        });
        stylesheet!(".dviz-sparkline-wash", {
            .raw("fill-opacity: 0; pointer-events: none;")
        });
        stylesheet!(".dviz-vivid .dviz-sparkline-wash", {
            .raw("fill-opacity: 0.6;")
        });
        stylesheet!(".dviz-sparkline-end", {
            .raw("fill: var(--dviz-series-1);")
        });
        stylesheet!(".dviz-figure", {
            .raw("display: flex; flex-direction: column; gap: 8px; min-width: 0;")
        });
        stylesheet!(".dviz-figure-row", {
            .raw("flex-direction: row; align-items: center; gap: 24px; flex-wrap: wrap;")
        });
        stylesheet!(".dviz-legend-items", {
            .raw("display: flex; flex-wrap: wrap; gap: 6px 8px; font-size: 12px; color: var(--dviz-ink-2);")
        });
        stylesheet!(".dviz-legend-item", {
            .raw("display: inline-flex; align-items: center; gap: 7px; padding: 3px 10px 3px 8px; border-radius: 999px; background: color-mix(in oklab, var(--dviz-ink) 5%, transparent); transition: background 150ms ease-out, opacity 150ms ease-out;")
        });
        stylesheet!(".dviz-legend-item:hover", {
            .raw("background: color-mix(in oklab, var(--dviz-ink) 10%, transparent);")
        });
        stylesheet!(".dviz-legend-swatch", {
            .raw("display: inline-block; width: 9px; height: 9px; border-radius: 50%; box-shadow: 0 0 0 2px color-mix(in oklab, currentColor 0%, transparent), 0 0 6px color-mix(in oklab, var(--dviz-ink) 0%, transparent);")
        });
        stylesheet!(".dviz-stat", {
            .raw("display: flex; flex-direction: column; gap: 4px; min-width: 0;")
        });
        stylesheet!(".dviz-stat-label", {
            .raw("font-size: 13px; color: var(--dviz-ink-2);")
        });
        stylesheet!(".dviz-stat-value", {
            .raw("font-family: var(--dviz-font-display, inherit); font-size: 34px; font-weight: 650; letter-spacing: -0.02em; line-height: 1.05; color: var(--dviz-ink); font-variant-numeric: normal;")
        });
        stylesheet!(".dviz-stat-delta", {
            .raw("display: flex; gap: 6px; align-items: baseline; font-family: var(--dviz-font-mono, inherit); font-size: 12px; color: var(--dviz-ink-2); font-variant-numeric: tabular-nums;")
        });
        stylesheet!(".dviz-stat-delta[data-tone=good] > span:first-child", {
            .raw("color: var(--dviz-delta-up-good); font-weight: 600;")
        });
        stylesheet!(".dviz-stat-delta[data-tone=bad] > span:first-child", {
            .raw("color: var(--dviz-status-critical); font-weight: 600;")
        });
        stylesheet!(".dviz-stat-delta-label", {
            .raw("color: var(--dviz-ink-muted);")
        });
        stylesheet!(".dviz-wash", {
            .raw("fill-opacity: 0.1; pointer-events: none;")
        });
        stylesheet!(".dviz-vivid .dviz-wash, .dviz-vivid .dviz-area-solo", {
            .raw("fill-opacity: 1;")
        });
        stylesheet!(".dviz-plot-bg", {
            .raw("fill: none;")
        });
        stylesheet!(".dviz-vivid .dviz-plot-bg", {
            .raw("fill: color-mix(in oklab, var(--dviz-ink) 3.5%, transparent);")
        });
        stylesheet!(".dviz-vivid .dviz-grid line", {
            .raw("stroke: color-mix(in oklab, var(--dviz-ink) 8%, transparent);")
        });
        stylesheet!(".dviz-vivid .dviz-axis-x line, .dviz-vivid .dviz-axis-y line", {
            .raw("stroke: color-mix(in oklab, var(--dviz-ink) 14%, transparent);")
        });
        stylesheet!(".dviz-vivid .dviz-line", {
            .raw("filter: drop-shadow(0 0 5px color-mix(in oklab, currentColor 45%, transparent));")
        });
        stylesheet!(".light .dviz-vivid .dviz-line, .dviz-live-chart .dviz-line, .dviz-live-chart .dviz-crosshair-dot", {
            .raw("filter: none;")
        });
        stylesheet!(".dviz-live-chart .dviz-enter .dviz-line, .dviz-live-chart .dviz-enter .dviz-wash", {
            .raw("animation: none !important;")
        });
        stylesheet!(".dviz-vivid .dviz-enter .dviz-line", {
            .raw("stroke-dasharray: 1; animation: dviz-draw 900ms cubic-bezier(0.2, 0.8, 0.2, 1) both;")
        });
        stylesheet!(".dviz-vivid .dviz-enter .dviz-wash", {
            .raw("animation: dviz-fade-in 900ms ease-out both;")
        });
        stylesheet!(".dviz-bar", {
            .raw("transform-box: fill-box;")
        });
        stylesheet!(".dviz-vivid .dviz-enter .dviz-bar, .dviz-vivid .dviz-enter.dviz-bar", {
            .raw("animation: dviz-grow 550ms cubic-bezier(0.2, 0.8, 0.2, 1) both;")
        });
        stylesheet!(".dviz-vivid .dviz-enter .dviz-point", {
            .raw("transform-box: fill-box; transform-origin: center; animation: dviz-pop 400ms cubic-bezier(0.2, 0.8, 0.2, 1) both;")
        });
        stylesheet!(".dviz-vivid .dviz-enter .dviz-cell, .dviz-vivid .dviz-enter .dviz-arc, .dviz-vivid .dviz-enter .dviz-area", {
            .raw("animation: dviz-fade-in 500ms ease-out both;")
        });
        stylesheet!(".dviz-vivid .dviz-enter.dviz-wash, .dviz-vivid .dviz-enter.dviz-area", {
            .raw("animation: dviz-fade-in 900ms ease-out both;")
        });
        // Tweens: attribute changes on persistent marks animate through CSS.
        stylesheet!(".dviz-vivid .dviz-bar, .dviz-vivid .dviz-line, .dviz-vivid .dviz-wash, .dviz-vivid .dviz-area", {
            .raw("transition: d 360ms cubic-bezier(0.2, 0.8, 0.2, 1), fill 200ms ease-out;")
        });
        stylesheet!(".dviz-arc", {
            .raw("transform-box: view-box; transition: transform 220ms cubic-bezier(0.2, 0.8, 0.2, 1), stroke-width 220ms ease-out; cursor: pointer; outline: none;")
        });
        stylesheet!(".dviz-vivid .dviz-arc[data-hovered=true]", {
            .raw("transform: scale(1.045);")
        });
        stylesheet!(".dviz-vivid .dviz-arc", {
            .raw("filter: drop-shadow(0 2px 6px color-mix(in oklab, currentColor 35%, transparent));")
        });
        stylesheet!(".light .dviz-vivid .dviz-arc", {
            .raw("filter: none;")
        });
        stylesheet!(".dviz-donut-value", {
            .raw("fill: var(--dviz-ink); font-family: var(--dviz-font-display, inherit); font-weight: 650; letter-spacing: -0.02em; font-variant-numeric: normal;")
        });
        stylesheet!(".dviz-donut-label", {
            .raw("fill: var(--dviz-ink-2); font-family: var(--dviz-font-mono, inherit); font-size: 11px;")
        });
        stylesheet!(".dviz-legend-table", {
            .raw("display: grid; grid-template-columns: auto 1fr auto auto; column-gap: 12px; row-gap: 2px; align-content: center; font-size: 13px; color: var(--dviz-ink-2); min-width: 200px;")
        });
        stylesheet!(".dviz-legend-row", {
            .raw("display: grid; grid-template-columns: subgrid; grid-column: 1 / -1; align-items: center; padding: 4px 8px; border-radius: 6px; cursor: pointer; transition: background 150ms ease-out, opacity 150ms ease-out;")
        });
        stylesheet!(".dviz-legend-row:hover", {
            .raw("background: color-mix(in oklab, var(--dviz-ink) 7%, transparent);")
        });
        stylesheet!(".dviz-legend-row:focus-visible", {
            .raw("outline: 2px solid var(--dviz-accent); outline-offset: 1px;")
        });
        stylesheet!(".dviz-legend-row[aria-pressed=false]", {
            .raw("opacity: 0.45;")
        });
        stylesheet!(".dviz-legend-row[aria-pressed=false] .dviz-legend-name", {
            .raw("text-decoration: line-through;")
        });
        stylesheet!(".dviz-legend-name", {
            .raw("color: var(--dviz-ink); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;")
        });
        stylesheet!(".dviz-legend-value, .dviz-legend-share", {
            .raw("font-family: var(--dviz-font-mono, inherit); font-variant-numeric: tabular-nums; text-align: right;")
        });
        stylesheet!(".dviz-legend-share", {
            .raw("color: var(--dviz-ink-muted);")
        });
        stylesheet!(".dviz-vivid .dviz-point, .dviz-vivid .dviz-hit, .dviz-vivid .dviz-end-marker, .dviz-vivid .dviz-halo", {
            .raw("transition: cx 360ms cubic-bezier(0.2, 0.8, 0.2, 1), cy 360ms cubic-bezier(0.2, 0.8, 0.2, 1);")
        });
        stylesheet!(".dviz-vivid .dviz-end", {
            .raw("transition: opacity 200ms ease-out;")
        });
        stylesheet!(".dviz-vivid .dviz-cell", {
            .raw("transition: x 360ms cubic-bezier(0.2, 0.8, 0.2, 1), y 360ms cubic-bezier(0.2, 0.8, 0.2, 1), width 360ms cubic-bezier(0.2, 0.8, 0.2, 1), height 360ms cubic-bezier(0.2, 0.8, 0.2, 1), fill 360ms ease-out;")
        });
        stylesheet!(".dviz-vivid .dviz-tick", {
            .raw("transition: transform 360ms cubic-bezier(0.2, 0.8, 0.2, 1), opacity 200ms ease-out;")
        });
        stylesheet!(".dviz-vivid .dviz-tick.dviz-enter", {
            .raw("animation: dviz-fade-in 240ms ease-out both;")
        });
        // Exits: a leaving node fades; a leaving bar also sinks to its baseline.
        stylesheet!(".dviz-leave", {
            .raw("opacity: 0 !important; pointer-events: none; transition: opacity 220ms ease-out;")
        });
        stylesheet!(".dviz-vivid .dviz-leave .dviz-bar, .dviz-vivid .dviz-leave.dviz-bar", {
            .raw("transform: scaleY(0.001); transition: transform 240ms ease-in, opacity 220ms ease-out;")
        });
        // Live charts: never lag the data.
        stylesheet!(".dviz-live-chart .dviz-bar, .dviz-live-chart .dviz-line, .dviz-live-chart .dviz-wash, .dviz-live-chart .dviz-area, .dviz-live-chart .dviz-arc, .dviz-live-chart .dviz-point, .dviz-live-chart .dviz-hit, .dviz-live-chart .dviz-end-marker, .dviz-live-chart .dviz-halo, .dviz-live-chart .dviz-cell, .dviz-live-chart .dviz-tick", {
            .raw("transition: none !important;")
        });
        stylesheet!(".dviz-live-chart .dviz-tick.dviz-enter", {
            .raw("animation: none !important;")
        });
        stylesheet!(".dviz-halo", {
            .raw("fill: none; stroke: currentColor; stroke-width: 2; pointer-events: none; animation: dviz-halo 1.6s ease-out infinite;")
        });
        stylesheet!(".dviz-crosshair-group", {
            .raw("transition: opacity 120ms ease-out; pointer-events: none;")
        });
        stylesheet!(".dviz-vivid .dviz-crosshair", {
            .raw("stroke: color-mix(in oklab, var(--dviz-ink) 35%, transparent);")
        });
        stylesheet!(".dviz-vivid .dviz-crosshair-dot", {
            .raw("filter: drop-shadow(0 0 4px color-mix(in oklab, currentColor 70%, transparent));")
        });
        stylesheet!(".dviz-end-marker", {
            .raw("stroke: var(--dviz-surface); stroke-width: 2; pointer-events: none;")
        });
        stylesheet!(".dviz-lines > g, .dviz-bars > g, .dviz-areas > g, .dviz-points > g", {
            .raw("transition: opacity 150ms ease-out;")
        });
        stylesheet!(".dviz-bar:hover, .dviz-cell:hover", {
            .raw("filter: brightness(1.15);")
        });
        stylesheet!(".dviz-hit:hover + .dviz-point, .dviz-point:has(+ .dviz-hit:hover)", {
            .raw("filter: brightness(1.15);")
        });
        stylesheet!(".dviz-tooltip", {
            .raw("opacity: 0; transform: translateY(4px); transition: opacity 120ms ease-out, transform 160ms cubic-bezier(0.2, 0.8, 0.2, 1);")
        });
        stylesheet!(".dviz-tooltip[data-visible=true]", {
            .raw("opacity: 1; transform: translateY(0);")
        });
        stylesheet!(".dviz-tooltip-title:empty", {
            .raw("display: none;")
        });
        stylesheet!(".dviz-tooltip-host", {
            .raw("position: absolute; inset: 0; pointer-events: none;")
        });
        stylesheet!(".dviz-tooltip", {
            .raw("position: absolute; z-index: 30; min-width: 120px; max-width: 280px; padding: 8px 10px; border-radius: 8px; background: color-mix(in oklab, var(--dviz-surface) 82%, transparent); backdrop-filter: blur(10px) saturate(1.3); -webkit-backdrop-filter: blur(10px) saturate(1.3); color: var(--dviz-ink); font-size: 12px; line-height: 1.45; box-shadow: 0 8px 24px rgb(0 0 0 / 0.28), 0 0 0 1px color-mix(in oklab, var(--dviz-ink) 14%, transparent); pointer-events: none;")
        });
        stylesheet!(".dviz-tooltip-title", {
            .raw("font-family: var(--dviz-font-mono, inherit); font-weight: 600; font-size: 11px; color: var(--dviz-ink-2); margin-bottom: 4px; font-variant-numeric: tabular-nums;")
        });
        stylesheet!(".dviz-tooltip-row", {
            .raw("display: flex; align-items: center; gap: 6px;")
        });
        stylesheet!(".dviz-tooltip-swatch", {
            .raw("width: 8px; height: 8px; border-radius: 2px; flex: none;")
        });
        stylesheet!(".dviz-tooltip-label", {
            .raw("color: var(--dviz-ink-2); flex: 1 1 auto; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;")
        });
        stylesheet!(".dviz-tooltip-value", {
            .raw("font-family: var(--dviz-font-mono, inherit); font-variant-numeric: tabular-nums; font-weight: 600; margin-left: auto;")
        });
        stylesheet!(".dviz-overlay", {
            .raw("fill: none; pointer-events: all; cursor: crosshair; outline: none;")
        });
        stylesheet!(".dviz-overlay:focus-visible", {
            .raw("outline: 2px solid var(--dviz-accent); outline-offset: 2px;")
        });
        stylesheet!(".dviz-crosshair", {
            .raw("stroke: var(--dviz-ink-muted); stroke-width: 1; pointer-events: none;")
        });
        stylesheet!(".dviz-crosshair-dot", {
            .raw("stroke: var(--dviz-surface); stroke-width: 2; pointer-events: none;")
        });
        stylesheet!(".dviz-brush", {
            .raw("fill: var(--dviz-accent); fill-opacity: 0.12; stroke: var(--dviz-accent); stroke-width: 1; pointer-events: none;")
        });
        stylesheet!(".dviz-hit", {
            .raw("fill: transparent; pointer-events: all;")
        });
        stylesheet!(".dviz-bar:focus-visible, .dviz-cell:focus-visible, .dviz-arc:focus-visible, .dviz-hit:focus-visible", {
            .raw("outline: 2px solid var(--dviz-ink); outline-offset: 1px;")
        });
        stylesheet!(".dviz-legend-item[role=button]", {
            .raw("cursor: pointer; border: 0; color: inherit; font: inherit;")
        });
        stylesheet!(".dviz-legend-item[role=button]:focus-visible", {
            .raw("outline: 2px solid var(--dviz-accent); outline-offset: 1px;")
        });
        stylesheet!(".dviz-legend-item[aria-pressed=false]", {
            .raw("opacity: 0.45; text-decoration: line-through;")
        });
        stylesheet!(".dviz-zoom-reset", {
            .raw("align-self: flex-start; font-family: var(--dviz-font-mono, inherit); font-size: 12px; padding: 2px 8px; border-radius: 4px; border: 1px solid var(--dviz-axis); background: none; color: var(--dviz-ink-2); cursor: pointer; transition: border-color 150ms ease-out, color 150ms ease-out;")
        });
        stylesheet!(".dviz-zoom-reset:hover, .dviz-live:hover", {
            .raw("border-color: var(--dviz-accent); color: var(--dviz-ink);")
        });
        stylesheet!(".dviz-zoom-reset:focus-visible, .dviz-live:focus-visible", {
            .raw("outline: 2px solid var(--dviz-accent); outline-offset: 2px;")
        });
        stylesheet!(".dviz-live", {
            .raw("display: inline-flex; align-items: center; gap: 6px; font-family: var(--dviz-font-mono, inherit); font-size: 12px; font-weight: 600; padding: 2px 8px; border-radius: 999px; border: 1px solid var(--dviz-axis); background: none; color: var(--dviz-ink-2); cursor: pointer; transition: border-color 150ms ease-out, color 150ms ease-out;")
        });
        stylesheet!(".dviz-live-dot", {
            .raw("width: 8px; height: 8px; border-radius: 50%; background: var(--dviz-ink-muted);")
        });
        stylesheet!(".dviz-live[data-live=true] .dviz-live-dot", {
            .raw("background: var(--dviz-status-good); animation: dviz-pulse 1.4s ease-in-out infinite;")
        });
        stylesheet!(".dviz-multiples", {
            .raw("display: grid; gap: 16px; min-width: 0;")
        });
        stylesheet!(".dviz-multiple-title", {
            .raw("font-family: var(--dviz-font-mono, inherit); font-size: 11px; font-weight: 500; color: var(--dviz-ink-2); margin-bottom: 4px;")
        });
    });
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn stylesheet_installs_once() {
        apply_style_sheet();
        let document = web_sys::window().unwrap().document().unwrap();
        let before = document.style_sheets().length();
        apply_style_sheet();
        assert_eq!(document.style_sheets().length(), before);
        assert!(before >= 1);
    }

    #[wasm_bindgen_test]
    fn declarations_cover_every_slot() {
        let d = declarations(Mode::Light);
        for i in 1..=SERIES_SLOTS {
            assert!(d.contains(&format!("--dviz-series-{i}:")));
        }
        assert!(d.contains("--dviz-series-1: #2a78d6;"));
        assert!(declarations(Mode::Dark).contains("--dviz-series-1: #3987e5;"));
        assert_eq!(series_var(0), "var(--dviz-series-1)");
        assert_eq!(series_var(8), "var(--dviz-ink-muted)");
    }
}
