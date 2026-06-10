use std::sync::atomic::{AtomicU64, Ordering};

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
