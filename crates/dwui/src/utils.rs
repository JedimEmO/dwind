use std::sync::atomic::{AtomicU64, Ordering};

/// Renders `content` as a direct child of `document.body` for as long as the
/// returned placeholder stays mounted.
///
/// `position: fixed` is relative to the nearest *transformed* ancestor, not
/// the viewport, so overlays rendered in place break as soon as any ancestor
/// carries a transform, filter, or backdrop-filter (page-transition
/// animations are a common source). Portalling the overlay to the body keeps
/// it viewport-anchored no matter where the component is used. Unmounting the
/// placeholder removes the portalled content.
pub fn body_portal(content: dominator::Dom) -> dominator::Dom {
    use discard::Discard;
    use dominator::{html, DomHandle};
    use std::cell::RefCell;
    use std::rc::Rc;

    let handle: Rc<RefCell<Option<DomHandle>>> = Rc::new(RefCell::new(None));

    html!("div", {
        .style("display", "none")
        .after_inserted({
            let handle = handle.clone();
            move |_| {
                let document = web_sys::window()
                    .expect("no window")
                    .document()
                    .expect("no document");

                // Apps sometimes replace <body> wholesale (dominator's
                // replace_dom pattern), so fall back to the document element.
                let target: web_sys::Element = document
                    .body()
                    .map(Into::into)
                    .or_else(|| document.document_element())
                    .expect("no element to portal into");

                handle
                    .borrow_mut()
                    .replace(dominator::append_dom(target.as_ref(), content));
            }
        })
        .after_removed(move |_| {
            if let Some(handle) = handle.borrow_mut().take() {
                handle.discard();
            }
        })
    })
}

/// Generates a document-unique element id with the given prefix.
///
/// Used to associate `<label for=..>`, `aria-describedby`, and similar
/// relationships between elements of a single component instance.
pub fn component_id(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    format!(
        "dwui-{}-{}",
        prefix,
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_ids_are_unique() {
        let a = component_id("input");
        let b = component_id("input");

        assert_ne!(a, b);
        assert!(a.starts_with("dwui-input-"));
    }
}
