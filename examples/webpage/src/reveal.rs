//! Scroll-triggered progressive reveal.
//!
//! A single shared `IntersectionObserver` watches tagged elements; when one
//! scrolls into view it gains `reveal-in`, and the child-selector variants
//! applied by [`reveal_on_scroll`] cascade its children up with a stagger.
//! Elements are unobserved after revealing, so the effect plays once.
//!
//! The whole cascade — including the parent-state-driven `.reveal-in > *` rules
//! and the `:nth-child` stagger — is expressed with `dwclass!` variants, so it
//! needs no stylesheet.

use dominator::DomBuilder;
use dwind::prelude::*;
use dwind_macros::dwclass;
use std::cell::RefCell;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::js_sys::Array;
use web_sys::{
    Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

thread_local! {
    static OBSERVER: RefCell<Option<IntersectionObserver>> = const { RefCell::new(None) };
}

fn with_observer(f: impl FnOnce(&IntersectionObserver)) {
    OBSERVER.with(|cell| {
        let mut cell = cell.borrow_mut();

        if cell.is_none() {
            let callback = Closure::<dyn FnMut(Array, IntersectionObserver)>::new(
                |entries: Array, observer: IntersectionObserver| {
                    for entry in entries.iter() {
                        let entry: IntersectionObserverEntry = entry.unchecked_into();

                        if entry.is_intersecting() {
                            let target = entry.target();
                            let _ = target.class_list().add_1("reveal-in");
                            observer.unobserve(&target);
                        }
                    }
                },
            );

            let options = IntersectionObserverInit::new();
            options.set_threshold(&JsValue::from_f64(0.15));

            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                    .unwrap_throw();

            // the observer lives for the page lifetime
            callback.forget();
            *cell = Some(observer);
        }

        f(cell.as_ref().unwrap())
    })
}

/// Tags the element for scroll reveal. Apply on a container; its direct
/// children fade up in a staggered cascade when it enters the viewport.
pub fn reveal_on_scroll(builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    builder
        // Resting state: every direct child is down and invisible.
        .apply(|b| {
            dwclass!(
                b,
                "[& > *]:opacity-0 [& > *]:[transform:translateY(26px)] \
                 [& > *]:[transition:opacity 650ms cubic-bezier(0.16, 1, 0.3, 1), transform 650ms cubic-bezier(0.16, 1, 0.3, 1)]"
            )
        })
        // Stagger. Each child leaves a beat after the one before it.
        .apply(|b| {
            dwclass!(
                b,
                "[& > *:nth-child(2)]:delay-75 [& > *:nth-child(3)]:delay-150 \
                 [& > *:nth-child(4)]:[transition-delay:210ms] [& > *:nth-child(5)]:[transition-delay:280ms]"
            )
        })
        // Revealed state, switched on by the observer adding `reveal-in`.
        .apply(|b| {
            dwclass!(
                b,
                "[&.reveal-in > *]:opacity-100 [&.reveal-in > *]:[transform:translateY(0)]"
            )
        })
        .after_inserted(|element| {
            let element: &Element = element.as_ref();
            with_observer(|observer| observer.observe(element));
        })
}
