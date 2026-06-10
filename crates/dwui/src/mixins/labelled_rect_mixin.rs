use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use dominator::{html, DomBuilder};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{not, Signal};
use web_sys::HtmlElement;

/// Draws the floating label, outline rectangle, and validation message for
/// labelled input components.
///
/// `input_id` is the id of the inner form control; it associates the
/// `<label for=..>` element and the validation message
/// (`id = {input_id}-error`) which the control should reference via
/// `aria-describedby` when invalid.
pub fn labelled_rect_mixin(
    label: impl Signal<Item = String> + 'static,
    raise_label: impl Signal<Item = bool> + 'static,
    validation_signal: impl Signal<Item = ValidationResult> + 'static,
    input_id: String,
) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |b| {
        let raise_label = raise_label.broadcast();
        let label = label.broadcast();
        let validation_signal = validation_signal.broadcast();

        let is_valid = validation_signal
            .signal_ref(|validation| match validation {
                ValidationResult::Valid => true,
                ValidationResult::Invalid { .. } => false,
            })
            .broadcast();

        let top_border_margin_signal = map_ref! {
            let raise = raise_label.signal(),
            let label = label.signal_cloned() => {
                if !raise {
                    "0px".to_string()
                } else {
                    format!("{}px", label.len() as f32  * 9.)
                }
            }
        };

        let error_id = format!("{}-error", input_id);

        b.child(html!("label", {
            .attr("for", &input_id)
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all m-l-4")
            .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
            .dwclass_signal!("text-sm", raise_label.signal())
            .dwclass_signal!("text-base", not(raise_label.signal()))
            .style_signal("margin-top", raise_label.signal().map(|v| {
                if v {
                    "-10px"
                } else {
                    "8px"
                }
            }))
            .text_signal(label.signal_cloned())
        }))
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none border-l border-r border-b rounded-b-sm transition-colors")
            .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
            .dwclass_signal!("dwui-border-error-600 is(.light *):dwui-border-error-700", not(is_valid.signal()))
        }))
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none w-2 border-t transition-colors")
            .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
            .dwclass_signal!("dwui-border-error-500 is(.light *):dwui-border-error-700", not(is_valid.signal()))
        }))
            .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all border-t transition-colors")
            .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
            .dwclass_signal!("dwui-border-error-500 is(.light *):dwui-border-error-700", not(is_valid.signal()))
            .style_signal("margin-left", top_border_margin_signal)
        }))
        .child_signal(validation_signal.signal_cloned().map(move |validation| {
            match validation {
                ValidationResult::Valid => None,
                ValidationResult::Invalid { message } => {
                    Some(html!("div", {
                        .attr("id", &error_id)
                        .attr("role", "alert")
                        .dwclass!("grid-col-1 grid-row-2 pointer-events-none transition-all text-sm h-4")
                        .dwclass!("dwui-text-error-500 is(.light *):dwui-text-error-700")
                        .text(&message)
                    }))
                }
            }
        }))
    }
}
