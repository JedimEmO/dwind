use crate::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProgressVariant {
    Accent,
    Success,
    Warning,
    Error,
}

#[component(render_fn = glass_progress_bar)]
struct GlassProgressBar {
    /// Progress value from 0.0 to 1.0.
    #[signal]
    #[default(0.0)]
    value: f64,

    #[signal]
    #[default(None)]
    label: Option<String>,

    #[signal]
    #[default(false)]
    show_percentage: bool,

    #[signal]
    #[default(ProgressVariant::Accent)]
    variant: ProgressVariant,
}

pub fn glass_progress_bar(props: GlassProgressBarProps) -> Dom {
    let GlassProgressBarProps {
        value,
        label,
        show_percentage,
        variant,
        apply,
    } = props;

    let value = value.broadcast();
    let variant = variant.broadcast();

    html!("div", {
        .dwclass!("flex flex-col gap-1")

        // Label row
        .child(html!("div", {
            .dwclass!("flex items-center justify-between")
            .child_signal(label.map(|l| {
                l.map(|text| html!("span", {
                    .dwclass!("text-sm glass-text-secondary")
                    .text(&text)
                }))
            }))
            .child_signal({
                let value = value.clone();
                futures_signals::map_ref! {
                    let show = show_percentage,
                    let v = value.signal() => {
                        if *show {
                            Some(html!("span", {
                                .dwclass!("text-sm glass-text-secondary")
                                .text(&format!("{}%", (*v * 100.0).round() as u32))
                            }))
                        } else {
                            None
                        }
                    }
                }
            })
        }))

        // Track
        .child(html!("div", {
            .style("background", "var(--glass-bg-inset)")
            .style("border-radius", "var(--glass-border-radius-full)")
            .style("border", "none")
            .style("box-shadow", "var(--glass-shadow-inset)")
            .style("height", "8px")
            .style("overflow", "hidden")

            .attr("role", "progressbar")
            .attr_signal("aria-valuenow", value.signal().map(|v| {
                format!("{}", (v * 100.0).round() as u32)
            }))
            .attr("aria-valuemin", "0")
            .attr("aria-valuemax", "100")

            // Fill
            .child(html!("div", {
                .style("height", "100%")
                .style("border-radius", "var(--glass-border-radius-full)")
                .style("transition-duration", "var(--glass-transition)")
                .style("transition-timing-function", "var(--glass-ease)")
                .style("transition-property", "width")

                .style_signal("width", value.signal().map(|v| {
                    format!("{}%", (v.clamp(0.0, 1.0) * 100.0).round())
                }))

                .style_signal("background", variant.signal().map(|v| match v {
                    ProgressVariant::Accent => "var(--glass-accent)",
                    ProgressVariant::Success => "var(--glass-success)",
                    ProgressVariant::Warning => "var(--glass-warning)",
                    ProgressVariant::Error => "var(--glass-error)",
                }))
            }))
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
