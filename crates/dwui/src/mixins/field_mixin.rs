use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use dominator::{html, Dom, DomBuilder};
use dwind::prelude::*;
use futures_signals::signal::not;
use futures_signals::signal::Signal;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;

/// Styles a field surface and draws its floating label.
///
/// Apply this to the element that wraps the form control. The surface gets the
/// filled-field look (`dwui-field-surface`, so `:focus-within` draws the focus
/// ring from the base stylesheet), a state-colored underline, and a label that
/// floats between the field's center and its top edge purely via `transform` —
/// it works for any label length, font, or script.
///
/// The control inside the surface should be transparent (`bg-transparent`),
/// fill the surface (`w-full h-full`), and leave room for the raised label
/// (`p-t-3`). Pair the surface with [`field_error_row`] so validation messages
/// have reserved space below the field.
///
/// `input_id` is the id of the inner form control; it associates the
/// `<label for=..>` element.
pub fn field_surface_mixin(
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
            .signal_ref(|validation| validation.is_valid())
            .broadcast();

        let b = dwclass!(
            b,
            "dwui-field-surface relative w-full rounded-md overflow-hidden"
        );
        let b = dwclass!(b, "dwui-bg-void-900 is(.light *):dwui-bg-void-200");
        let b = dwclass!(b, "border-b-2 transition-colors");
        let b = dwclass_signal!(
            b,
            "dwui-border-void-500 is(.light *):dwui-border-void-400",
            is_valid.signal()
        );
        let b = dwclass_signal!(
            b,
            "dwui-border-error-500 is(.light *):dwui-border-error-600",
            not(is_valid.signal())
        );

        b.child(html!("label", {
            .attr("for", &input_id)
            .dwclass!("absolute pointer-events-none select-none p-l-3 text-base")
            .dwclass_signal!("dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600", is_valid.signal())
            .dwclass_signal!("dwui-text-error-500 is(.light *):dwui-text-error-600", not(is_valid.signal()))
            .style("left", "0")
            // A tight line box: with the default ~1.5 line-height the raised
            // label reaches the surface's top edge and collides with the
            // :focus-within outline drawn just inside it.
            .style("line-height", "1")
            // Multi-line surfaces (textarea) override the resting anchor so the
            // label rests on their first line instead of the surface's center.
            .style("top", "var(--dwui-field-label-top, 50%)")
            .style("transform-origin", "left top")
            .style("transition", "transform 150ms ease, color 150ms ease")
            .style_signal("transform", raise_label.signal().map(|raised| {
                if raised {
                    // Float to the top edge of the surface, shrunk. Pure
                    // transform: no measuring, so any label length or script
                    // works.
                    "translateY(-100%) scale(0.75)"
                } else {
                    // Vertically centered resting position.
                    "translateY(-50%)"
                }
            }))
            .text_signal(label.signal_cloned())
        }))
    }
}

/// The validation message row belonging to a field surface.
///
/// Always present in the DOM with a reserved height, so a field switching
/// between valid and invalid never shifts the surrounding layout. The row only
/// takes `role="alert"` while it has a message, and carries
/// `id = {input_id}-error` so the control can point at it via
/// `aria-describedby`.
pub fn field_error_row(
    validation_signal: impl Signal<Item = ValidationResult> + 'static,
    input_id: &str,
) -> Dom {
    let message = validation_signal
        .map(|validation| match validation {
            ValidationResult::Valid => "".to_string(),
            ValidationResult::Invalid { message } => message,
        })
        .broadcast();

    html!("div", {
        .attr("id", &format!("{}-error", input_id))
        .attr_signal("role", message.signal_ref(|m| {
            if m.is_empty() {
                None
            } else {
                Some("alert")
            }
        }))
        .dwclass!("min-h-5 text-sm p-l-3 pointer-events-none")
        .dwclass!("dwui-text-error-500 is(.light *):dwui-text-error-700")
        .text_signal(message.signal_cloned())
    })
}
