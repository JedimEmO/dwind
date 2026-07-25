//! The reactivity lab.
//!
//! Three `Mutable`s drive eleven live CSS properties across nine nodes, and the
//! Rust that would produce them is regenerated as you drag. This is the whole
//! pitch in one panel: there is no virtual DOM pass, no re-render, no diff —
//! a slider writes a number and DOMINATOR writes exactly the properties that
//! depend on it.

use crate::fx;
use dominator::{html, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};

pub fn signal_lab() -> Dom {
    // The entire state of the demo. Everything below is derived.
    let spread = Mutable::new(58.0f32);
    let twist = Mutable::new(34.0f32);
    let glow = Mutable::new(46.0f32);
    let stacked = Mutable::new(true);

    // A cheap honesty check: bumped once per property write, so the counter in
    // the corner is a real measurement rather than a marketing number.
    let writes = Mutable::new(0u32);

    html!("section", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-20 w-full")
        .apply(crate::reveal::reveal_on_scroll)
        .child(super::home::section_header(
            "reactivity",
            "Move a slider. Watch the DOM barely flinch.",
        ))
        .child(html!("div", {
            .dwclass!("grid @md:grid-cols-5 @<md:grid-cols-1 gap-4 m-t-8")
            .child(html!("div", {
                .dwclass!("@md:col-span-3 @<md:col-span-1")
                .child(stage(&spread, &twist, &glow, &stacked, &writes))
            }))
            .child(html!("div", {
                .dwclass!("@md:col-span-2 @<md:col-span-1 flex flex-col gap-4")
                .child(controls(&spread, &twist, &glow, &stacked))
                .child(generated_code(&spread, &twist, &glow))
            }))
        }))
    })
}

// ---------------------------------------------------------------------------
// Stage — the thing that actually moves
// ---------------------------------------------------------------------------

fn stage(
    spread: &Mutable<f32>,
    twist: &Mutable<f32>,
    glow: &Mutable<f32>,
    stacked: &Mutable<bool>,
    writes: &Mutable<u32>,
) -> Dom {
    const CARD_TINTS: [&str; 5] = ["#D5B65F", "#D59A5F", "#5FB0D5", "#8F5FD5", "#5FD59A"];

    html!("div", {
        .class("dw-glass")
        .dwclass!("rounded-lg border border-woodsmoke-800 overflow-hidden")
        .style("position", "relative")
        .style("min-height", "22rem")
        .child(fx::blueprint_grid("radial-gradient(ellipse 80% 80% at 50% 50%, black 20%, transparent 78%)"))
        // the fan of cards
        .child(html!("div", {
            .dwclass!("flex flex-row justify-center align-items-center w-full")
            .style("position", "relative")
            .style("height", "22rem")
            .style("perspective", "1200px")
            .children((0..5).map(|i| {
                fan_card(i, CARD_TINTS[i], spread, twist, glow, stacked, writes)
            }))
        }))
        .child(write_counter(writes))
    })
}

#[allow(clippy::too_many_arguments)]
fn fan_card(
    index: usize,
    tint: &'static str,
    spread: &Mutable<f32>,
    twist: &Mutable<f32>,
    glow: &Mutable<f32>,
    stacked: &Mutable<bool>,
    writes: &Mutable<u32>,
) -> Dom {
    // -2 .. 2, so the fan opens symmetrically around the middle card.
    let offset = index as f32 - 2.0;

    let transform = map_ref! {
        let spread = spread.signal(),
        let twist = twist.signal(),
        let stacked = stacked.signal() => move {
            let fan = if *stacked { 1.0 } else { 0.15 };

            format!(
                "translateX({:.1}px) translateY({:.1}px) rotate({:.2}deg) rotateY({:.2}deg) scale({:.3})",
                offset * spread * fan,
                offset.abs() * spread * 0.16 * fan,
                offset * twist * 0.22 * fan,
                offset * twist * 0.5 * fan,
                1.0 - offset.abs() * 0.035,
            )
        }
    };

    html!("div", {
        .dwclass!("rounded-lg border border-woodsmoke-700 flex flex-col justify-between p-4")
        .style("position", "absolute")
        .style("width", "9.5rem")
        .style("height", "13rem")
        .style("background", "linear-gradient(160deg, rgba(30, 30, 36, 0.96), rgba(10, 10, 13, 0.96))")
        .style("transition", "transform 260ms cubic-bezier(0.16, 1, 0.3, 1), box-shadow 260ms ease")
        .style("z-index", &(10 - index.abs_diff(2)).to_string())
        .style_signal("transform", {
            let writes = writes.clone();

            transform.map(move |t| {
                writes.set(writes.get().wrapping_add(1));
                t
            })
        })
        .style_signal("box-shadow", {
            let writes = writes.clone();
            let tint = tint.to_string();

            glow.signal().map(move |g| {
                writes.set(writes.get().wrapping_add(1));
                format!("0 0 {:.0}px -4px {}{:02X}", g * 0.9, tint, ((g / 100.0) * 190.0) as u8)
            })
        })
        .style_signal("border-color", glow.signal().map(move |g| {
            format!("{}{:02X}", tint, ((g / 100.0) * 150.0) as u8 + 30)
        }))
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs")
            .style("color", tint)
            .text(&format!("node[{index}]"))
        }))
        .child(html!("div", {
            .dwclass!("flex flex-col gap-2")
            .child(html!("div", {
                .dwclass!("h-1 rounded-full")
                .style("background", tint)
                .style("opacity", "0.55")
                .style_signal("width", spread.signal().map(move |s| {
                    format!("{:.0}%", 30.0 + (s * 0.6).min(65.0))
                }))
            }))
            .child(html!("div", {
                .dwclass!("h-1 rounded-full w-p-40 bg-woodsmoke-700")
            }))
        }))
    })
}

fn write_counter(writes: &Mutable<u32>) -> Dom {
    html!("div", {
        .class("font-code")
        .dwclass!("flex flex-row gap-3 align-items-center")
        .dwclass!("text-xs text-woodsmoke-500 border border-woodsmoke-800 rounded-full p-l-3 p-r-3 p-t-1 p-b-1")
        .style("position", "absolute")
        .style("bottom", "0.9rem")
        .style("right", "0.9rem")
        .style("background", "rgba(2, 2, 3, 0.7)")
        .child(html!("span", {
            .dwclass!("w-2 h-2 rounded-full bg-apple-400 flex-none")
            .style("animation", "dwind-pulse-ring 2.2s ease-out infinite")
        }))
        .child(html!("span", {
            .dwclass!("text-woodsmoke-400")
            .text_signal(writes.signal().map(|n| format!("{n} property writes")))
        }))
        .child(html!("span", { .text("· 0 re-renders") }))
    })
}

// ---------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------

fn controls(
    spread: &Mutable<f32>,
    twist: &Mutable<f32>,
    glow: &Mutable<f32>,
    stacked: &Mutable<bool>,
) -> Dom {
    let stacked = stacked.clone();

    html!("div", {
        .class("dw-glass")
        .dwclass!("rounded-lg border border-woodsmoke-800 p-6 flex flex-col gap-5")
        .apply(fx::spotlight)
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-500")
            .text("// Mutable<f32> × 3, Mutable<bool> × 1")
        }))
        .child(slider!({
            .value(spread.clone())
            .label("spread".to_string())
        }))
        .child(slider!({
            .value(twist.clone())
            .label("twist".to_string())
        }))
        .child(slider!({
            .value(glow.clone())
            .label("glow".to_string())
        }))
        .child(switch!({
            .checked_signal(stacked.signal())
            .label("fan out".to_string())
            .on_change(clone!(stacked => move |v| stacked.set(v)))
        }))
    })
}

// ---------------------------------------------------------------------------
// Live source view
// ---------------------------------------------------------------------------

/// The code panel is itself a signal — it re-derives from the same three
/// `Mutable`s the stage reads, so what you see is what is running.
fn generated_code(spread: &Mutable<f32>, twist: &Mutable<f32>, glow: &Mutable<f32>) -> Dom {
    let values = map_ref! {
        let spread = spread.signal(),
        let twist = twist.signal(),
        let glow = glow.signal() => (*spread, *twist, *glow)
    };

    html!("div", {
        .dwclass!("rounded-lg border border-woodsmoke-800 overflow-hidden")
        .style("background", "rgba(6, 6, 8, 0.8)")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-500 p-l-4 p-r-4 p-t-2 p-b-2 border-b border-woodsmoke-800")
            .text("stage.rs — live")
        }))
        .child(html!("pre", {
            .class("font-code")
            .dwclass!("text-xs m-0 p-4 leading-relaxed overflow-x-auto text-woodsmoke-400")
            .text_signal(values.map(|(spread, twist, glow)| {
                format!(
    ".style_signal(\"transform\", spread.signal().map(|s| {{\n    \
    format!(\"translateX({{:.1}}px) rotate({{:.2}}deg)\", i * s, i * t)\n\
    }}))\n\n\
    // current: spread = {spread:.0}, twist = {twist:.0}, glow = {glow:.0}"
                )
            }))
        }))
    })
}
