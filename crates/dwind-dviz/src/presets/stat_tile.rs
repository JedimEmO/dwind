use dominator::{Dom, html};
use dwind_dviz_core::format;
use futures_signals::signal::{Mutable, SignalExt};

fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches())
}
use futures_signals_component_macro::component;

use super::sparkline::{SparklineProps, sparkline};

/// A headline number: label, compact value, an optional signed delta whose
/// color is direction × whether up is good, and an optional trend
/// sparkline. When the story is one number, this beats a chart.
#[component(render_fn = stat_tile)]
struct StatTile {
    /// Sentence case, no trailing colon.
    #[default(String::new())]
    label: String,

    #[signal]
    #[default(f64::NAN)]
    value: f64,

    /// Formats the value; defaults to the compact figure format.
    #[default(None)]
    format: Option<Box<dyn Fn(f64) -> String>>,

    /// Change versus a named period, as a fraction (0.12 = +12%).
    #[signal]
    #[default(None)]
    delta: Option<f64>,

    /// What the delta compares against, e.g. "vs last week".
    #[default(String::new())]
    delta_label: String,

    /// Whether an increase is good news (revenue) or bad (latency).
    #[default(true)]
    up_is_good: bool,

    /// Recent values for the trend sparkline.
    #[signal]
    #[default(vec![])]
    trend: Vec<f64>,
}

pub fn stat_tile(props: StatTileProps) -> Dom {
    let StatTileProps {
        label,
        value,
        format: fmt,
        delta,
        delta_label,
        up_is_good,
        trend,
        apply,
    } = props;

    let fmt = fmt.unwrap_or_else(|| Box::new(format::compact));
    let trend = trend.broadcast();
    let delta = delta.broadcast();
    // The displayed value eases toward the real one over ~600ms, so a
    // changed figure is noticed. Skipped under reduced motion.
    let shown = Mutable::new(f64::NAN);
    let target = value.broadcast();
    let count_up = {
        let shown = shown.clone();
        target.signal().for_each(move |v| {
            let shown = shown.clone();
            async move {
                let from = shown.get();
                if !v.is_finite() || !from.is_finite() || reduced_motion() || !crate::theme::vivid()
                {
                    shown.set(v);
                    return;
                }
                let steps = 24;
                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    let eased = 1.0 - (1.0 - t).powi(3);
                    shown.set(from + (v - from) * eased);
                    gloo_timers::future::TimeoutFuture::new(25).await;
                }
                shown.set(v);
            }
        })
    };

    html!("div", {
        .class("dviz-stat")
        .child(html!("div", {
            .class("dviz-stat-label")
            .text(&label)
        }))
        .future(count_up)
        .child(html!("div", {
            .class("dviz-stat-value")
            .text_signal(shown.signal().map(move |v| if v.is_finite() { fmt(v) } else { "–".to_string() }))
        }))
        .child_signal(delta.signal().map(move |d| {
            let d = d?;
            let up = d > 0.0;
            let good = if d == 0.0 { None } else { Some(up == up_is_good) };
            let tone = match good {
                Some(true) => "good",
                Some(false) => "bad",
                None => "flat",
            };
            let arrow = if d > 0.0 { "↑" } else if d < 0.0 { "↓" } else { "→" };
            let text = format!("{} {}", arrow, format::signed(d * 100.0, 1).trim_start_matches('+')).replace("↑ ", "↑ +");
            Some(html!("div", {
                .class("dviz-stat-delta")
                .attr("data-tone", tone)
                .child(html!("span", {
                    .attr("aria-label", if up { "up" } else if d < 0.0 { "down" } else { "unchanged" })
                    .text(&format!("{}%", text))
                }))
                .apply_if(!delta_label.is_empty(), |b| b.child(html!("span", {
                    .class("dviz-stat-delta-label")
                    .text(&delta_label)
                })))
            }))
        }))
        .child_signal(trend.signal_ref(|t| !t.is_empty()).dedupe().map({
            let trend = trend.clone();
            move |has| {
                has.then(|| sparkline(SparklineProps::new()
                    .label("Trend".to_string())
                    .values_signal(trend.signal_cloned())))
            }
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
