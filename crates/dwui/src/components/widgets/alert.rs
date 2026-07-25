use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
    Neutral,
}

impl AlertVariant {
    fn icon_path(self) -> &'static str {
        match self {
            // info: "i" in circle
            AlertVariant::Info | AlertVariant::Neutral => "M8 7.5 V11.5 M8 4.5 V5",
            // success: checkmark
            AlertVariant::Success => "M4.5 8.5 L7 11 L11.5 5",
            // warning + error: exclamation
            AlertVariant::Warning | AlertVariant::Error => "M8 4.5 V9 M8 11 V11.5",
        }
    }
}

/// A status callout banner.
///
/// `Warning` and `Error` variants render with `role="alert"` (interrupting
/// announcement); the others use `role="status"` (polite). When `dismissible`
/// is set, a labelled close button hides the alert and invokes `on_dismiss`.
#[component(render_fn = alert)]
struct Alert {
    #[signal]
    #[default(AlertVariant::Info)]
    variant: AlertVariant,

    #[signal]
    #[default("".to_string())]
    title: String,

    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(false)]
    dismissible: bool,

    #[default(Box::new(|| {}))]
    on_dismiss: dyn Fn() -> () + 'static,
}

pub fn alert(props: AlertProps) -> Dom {
    let AlertProps {
        variant,
        title,
        content,
        dismissible,
        on_dismiss,
        apply,
    } = props;

    let variant = variant.broadcast();
    let title = title.broadcast();
    let dismissed = Mutable::new(false);
    let on_dismiss = Rc::new(on_dismiss);

    html!("div", {
        .visible_signal(dismissed.signal().map(|v| !v))
        .attr_signal("role", variant.signal().map(|v| match v {
            AlertVariant::Warning | AlertVariant::Error => "alert",
            _ => "status",
        }))
        .dwclass!("flex flex-row gap-3 p-4 rounded-md border w-full")
        .dwclass_signal!("dwui-border-primary-700 is(.light *):dwui-border-primary-300", variant.signal().map(|v| v == AlertVariant::Info))
        .dwclass_signal!("dwui-border-success-700 is(.light *):dwui-border-success-400", variant.signal().map(|v| v == AlertVariant::Success))
        .dwclass_signal!("dwui-border-warning-600 is(.light *):dwui-border-warning-400", variant.signal().map(|v| v == AlertVariant::Warning))
        .dwclass_signal!("dwui-border-error-700 is(.light *):dwui-border-error-400", variant.signal().map(|v| v == AlertVariant::Error))
        .dwclass_signal!("dwui-border-void-700 is(.light *):dwui-border-void-300", variant.signal().map(|v| v == AlertVariant::Neutral))
        .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-100")
        .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-900")
        // Icon
        .child(html!("div", {
            .dwclass!("flex-none flex align-items-start")
            .dwclass_signal!("dwui-text-primary-400 is(.light *):dwui-text-primary-600", variant.signal().map(|v| v == AlertVariant::Info))
            .dwclass_signal!("dwui-text-success-400 is(.light *):dwui-text-success-600", variant.signal().map(|v| v == AlertVariant::Success))
            .dwclass_signal!("dwui-text-warning-400 is(.light *):dwui-text-warning-600", variant.signal().map(|v| v == AlertVariant::Warning))
            .dwclass_signal!("dwui-text-error-400 is(.light *):dwui-text-error-600", variant.signal().map(|v| v == AlertVariant::Error))
            .dwclass_signal!("dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600", variant.signal().map(|v| v == AlertVariant::Neutral))
            .child(svg!("svg", {
                .attr("viewBox", "0 0 16 16")
                .attr("width", "20")
                .attr("height", "20")
                .attr("fill", "none")
                .attr("aria-hidden", "true")
                .child(svg!("circle", {
                    .attr("cx", "8")
                    .attr("cy", "8")
                    .attr("r", "7")
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.5")
                }))
                .child(svg!("path", {
                    .attr_signal("d", variant.signal().map(|v| v.icon_path()))
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.5")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                }))
            }))
        }))
        // Body
        .child(html!("div", {
            .dwclass!("flex flex-col gap-1 grow")
            .child_signal(title.signal_cloned().map(|title| {
                if title.is_empty() {
                    return None;
                }

                Some(html!("div", {
                    .dwclass!("font-bold")
                    .text(&title)
                }))
            }))
            .child_signal(content)
        }))
        // Dismiss button
        .child_signal(dismissible.map(clone!(dismissed => move |dismissible| {
            if !dismissible {
                return None;
            }

            Some(html!("button", {
                .attr("type", "button")
                .attr("aria-label", "Dismiss")
                .dwclass!("flex-none w-6 h-6 flex align-items-center justify-center rounded-full cursor-pointer")
                .dwclass!("bg-transparent border-none dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                .dwclass!("hover:dwui-bg-void-800 is(.light *):hover:dwui-bg-void-200 transition-colors")
                .dwclass!("dwui-focusable")
                .text("×")
                .event(clone!(dismissed, on_dismiss => move |_: events::Click| {
                    dismissed.set(true);
                    (on_dismiss)();
                }))
            }))
        })))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
