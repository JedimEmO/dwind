//! Scroll-triggered progressive reveal.
//!
//! A single shared `IntersectionObserver` watches elements tagged with the
//! `reveal-section` class; when one scrolls into view it gains `reveal-in`,
//! and the CSS in [`crate::APP_KEYFRAMES`] cascades its children up with a
//! small stagger. Elements are unobserved after revealing, so the effect
//! plays once.

use dominator::DomBuilder;
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
    builder.class("reveal-section").after_inserted(|element| {
        let element: &Element = element.as_ref();
        with_observer(|observer| observer.observe(element));
    })
}
