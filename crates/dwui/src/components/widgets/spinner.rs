use crate::theme::prelude::*;
use dominator::{html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpinnerSize {
    Small,
    Medium,
    Large,
}


/// A loading spinner exposed to assistive technology as `role="status"`.
#[component(render_fn = spinner)]
struct Spinner {
    #[signal]
    #[default(SpinnerSize::Medium)]
    size: SpinnerSize,

    /// Accessible label announced while loading
    #[signal]
    #[default("Loading".to_string())]
    label: String,
}

pub fn spinner(props: SpinnerProps) -> Dom {
    let SpinnerProps { size, label, apply } = props;

    let size = size.broadcast();

    html!("span", {
        .attr("role", "status")
        .attr_signal("aria-label", label)
        .dwclass!("inline-flex align-items-center justify-center")
        .dwclass!("dwui-text-primary-400 is(.light *):dwui-text-primary-600")
        .child(html!("span", {
            .dwclass!("inline-flex align-items-center justify-center")
            .dwclass_signal!("w-4 h-4", size.signal().map(|s| s == SpinnerSize::Small))
            .dwclass_signal!("w-6 h-6", size.signal().map(|s| s == SpinnerSize::Medium))
            .dwclass_signal!("w-10 h-10", size.signal().map(|s| s == SpinnerSize::Large))
            .style("animation", "dwui-spin 0.8s linear infinite")
            .child(svg!("svg", {
            .attr("viewBox", "0 0 24 24")
            .attr("width", "100%")
            .attr("height", "100%")
            .attr("fill", "none")
            .attr("aria-hidden", "true")
            .child(svg!("circle", {
                .attr("cx", "12")
                .attr("cy", "12")
                .attr("r", "10")
                .attr("stroke", "currentColor")
                .attr("stroke-width", "3")
                .attr("opacity", "0.25")
            }))
            .child(svg!("path", {
                .attr("d", "M22 12 A10 10 0 0 0 12 2")
                .attr("stroke", "currentColor")
                .attr("stroke-width", "3")
                .attr("stroke-linecap", "round")
            }))
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
