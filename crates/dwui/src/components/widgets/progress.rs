use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

/// An accessible progress bar.
///
/// Renders a `role="progressbar"` element with `aria-valuenow/min/max`.
/// `value` is clamped to `0.0..=100.0`. When `indeterminate` is set, the bar
/// shows a looping sweep animation and omits `aria-valuenow`, which is how
/// assistive technology expects indeterminate progress to be exposed.
#[component(render_fn = progress)]
struct Progress {
    #[signal]
    #[default(0.0)]
    value: f64,

    #[signal]
    #[default(false)]
    indeterminate: bool,

    /// Accessible name for the progress bar
    #[signal]
    #[default("Progress".to_string())]
    label: String,
}

pub fn progress(props: ProgressProps) -> Dom {
    let ProgressProps {
        value,
        indeterminate,
        label,
        apply,
    } = props;

    let value = value.map(|v| v.clamp(0.0, 100.0)).broadcast();
    let indeterminate = indeterminate.broadcast();

    let aria_valuenow = map_ref! {
        let value = value.signal(),
        let indeterminate = indeterminate.signal() => {
            if *indeterminate {
                None
            } else {
                Some(format!("{}", value))
            }
        }
    };

    html!("div", {
        .attr("role", "progressbar")
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .attr_signal("aria-valuenow", aria_valuenow)
        .attr_signal("aria-label", label)
        .dwclass!("w-full h-2 rounded-full overflow-hidden")
        .dwclass!("dwui-bg-void-700 is(.light *):dwui-bg-void-200")
        .child(html!("div", {
            .dwclass!("h-full rounded-full")
            .dwclass!("dwui-bg-primary-500 is(.light *):dwui-bg-primary-400")
            .style("transition", "width 200ms ease-out")
            .style_signal("width", map_ref! {
                let value = value.signal(),
                let indeterminate = indeterminate.signal() => {
                    if *indeterminate {
                        "40%".to_string()
                    } else {
                        format!("{}%", value)
                    }
                }
            })
            .style_signal("animation", indeterminate.signal().map(|v| {
                if v {
                    Some("dwui-progress-indeterminate 1.2s ease-in-out infinite")
                } else {
                    None
                }
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
