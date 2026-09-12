//! Realtime plumbing: one publish per animation frame, and a live/paused
//! indicator.
//!
//! A live chart should hold a fixed y domain (pass `y` to the preset) so
//! the axis does not jitter as samples arrive, and should not animate mark
//! transitions on data arrival. Lines here already update by patching one
//! path attribute, so nothing animates unless you add it.

use std::cell::Cell;
use std::rc::Rc;

use dominator::{Dom, html};
use dwind_dviz_data::WindowedSource;
use futures_signals::signal::SignalExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Commits `source` on the next animation frame after it becomes dirty, so
/// any number of pushes between frames cost one publish. Call once per
/// source; the closure lives as long as the source does.
pub fn commit_on_frame(source: &Rc<WindowedSource>) {
    let scheduled = Rc::new(Cell::new(false));
    let callback: Rc<Closure<dyn FnMut(f64)>> = Rc::new(Closure::new({
        let source = Rc::downgrade(source);
        let scheduled = scheduled.clone();
        move |_ts: f64| {
            scheduled.set(false);
            if let Some(s) = source.upgrade() {
                s.commit();
            }
        }
    }));
    source.set_notifier(move || {
        if scheduled.replace(true) {
            return;
        }
        let ok = web_sys::window()
            .and_then(|w| {
                w.request_animation_frame(callback.as_ref().as_ref().unchecked_ref())
                    .ok()
            })
            .is_some();
        if !ok {
            scheduled.set(false);
        }
    });
}

/// A pulsing dot with "Live" while `following` is true, "Paused" otherwise.
/// Clicking it toggles the source's follow state. Place it in the figure
/// header of a live chart so the state is never ambiguous.
pub fn live_indicator(source: &Rc<WindowedSource>) -> Dom {
    let source = source.clone();
    let following = source.follow_signal().broadcast();
    html!("button", {
        .class("dviz-live")
        .attr("type", "button")
        .attr_signal("aria-pressed", following.signal().map(|f| if f { "true" } else { "false" }))
        .attr_signal("data-live", following.signal().map(|f| if f { "true" } else { "false" }))
        .attr("aria-label", "Toggle live updates")
        .child(html!("span", {
            .class("dviz-live-dot")
            .attr("aria-hidden", "true")
        }))
        .text_signal(following.signal().map(|f| if f { "Live" } else { "Paused" }))
        .event(move |_: dominator::events::Click| source.toggle_follow())
    })
}
