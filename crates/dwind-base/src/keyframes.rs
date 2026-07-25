//! Runtime registry for `@keyframes` rules.
//!
//! dwind's utility classes are generated as single declaration blocks, and the
//! CSS binding pipeline has no notion of at-rules — so a `@keyframes` cannot be
//! expressed as a utility. Historically every crate worked around that by
//! hand-writing a `&str` blob and pushing it through
//! [`dominator::stylesheet_raw`] at startup, which meant no deduplication, no
//! collision detection, and every keyframe paying its cost whether or not the
//! page used it.
//!
//! This module is the shared alternative. A [`Keyframes`] is a `const`-
//! constructible handle that injects its rule the first time anything asks for
//! its name, and never again.
//!
//! Prefer declaring these with the `dwkeyframes!` macro rather than by hand —
//! it mints the matching `animate-*` utility class at the same time, so the
//! class name stays checked by rustc.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Every keyframe name injected so far, mapped to the body it was injected
/// with. Both constructors are `const`, so this needs no lazy-init machinery.
static REGISTRY: Mutex<BTreeMap<&'static str, &'static str>> = Mutex::new(BTreeMap::new());

/// A declared `@keyframes` rule.
///
/// The rule is injected lazily — constructing a `Keyframes` costs nothing, and
/// the stylesheet is only touched once something calls [`Keyframes::name`],
/// [`Keyframes::ensure`], or formats the handle with `{}`.
///
/// ```ignore
/// static FADE: Keyframes = Keyframes::new("app-fade", "from{opacity:0;}to{opacity:1;}");
///
/// // `Display` registers, so composed shorthands work without ceremony:
/// el.style("animation", &format!("{FADE} 600ms ease-out both"));
/// ```
pub struct Keyframes {
    name: &'static str,
    body: &'static str,
    injected: AtomicBool,
}

impl Keyframes {
    pub const fn new(name: &'static str, body: &'static str) -> Self {
        Self {
            name,
            body,
            injected: AtomicBool::new(false),
        }
    }

    /// Injects the rule if it has not been injected yet, then returns the CSS
    /// keyframe name for use in an `animation` shorthand.
    pub fn name(&self) -> &'static str {
        self.ensure();
        self.name
    }

    /// The keyframe name *without* injecting the rule.
    ///
    /// Only useful in `const` contexts. If you are building an `animation`
    /// value at runtime, use [`Keyframes::name`] or `{}` formatting instead so
    /// the rule actually reaches the document.
    pub const fn name_unregistered(&self) -> &'static str {
        self.name
    }

    pub const fn body(&self) -> &'static str {
        self.body
    }

    /// Injects the rule. Idempotent, and after the first call this is a single
    /// relaxed atomic load.
    pub fn ensure(&self) {
        if self.injected.load(Ordering::Relaxed) {
            return;
        }

        register(self.name, self.body);
        self.injected.store(true, Ordering::Relaxed);
    }

    /// The full rule text, for server-side rendering or debugging. Does not
    /// inject anything.
    pub fn css(&self) -> String {
        format!("@keyframes {} {{ {} }}", self.name, self.body)
    }
}

impl std::fmt::Display for Keyframes {
    /// Writes the keyframe name — and registers the rule as a side effect, so
    /// that `format!("{KEYFRAMES} 1s linear")` cannot produce a shorthand
    /// pointing at a rule that was never injected.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Injects `@keyframes {name} { {body} }` unless `name` is already registered.
///
/// Registering the same name twice with different bodies is a bug — two crates
/// have picked the same keyframe name and one is silently winning. In debug
/// builds that panics; in release the first registration stands.
pub fn register(name: &'static str, body: &'static str) {
    let mut registry = match REGISTRY.lock() {
        Ok(registry) => registry,
        // A poisoned lock means an earlier registration panicked. Injecting
        // styles is not worth propagating that.
        Err(poisoned) => poisoned.into_inner(),
    };

    if let Some(existing) = registry.get(name) {
        debug_assert!(
            *existing == body,
            "@keyframes `{name}` was registered twice with different bodies. \
             Give one of them a distinct name — dwkeyframes! namespaces by crate \
             unless you override it with #[name = \"...\"]."
        );

        return;
    }

    registry.insert(name, body);
    dominator::stylesheet_raw(format!("@keyframes {name} {{ {body} }}"));
}

/// Whether `name` has already been injected.
pub fn is_registered(name: &str) -> bool {
    match REGISTRY.lock() {
        Ok(registry) => registry.contains_key(name),
        Err(poisoned) => poisoned.into_inner().contains_key(name),
    }
}

/// Every rule injected so far, concatenated. Intended for prerendering.
pub fn registered_css() -> String {
    let registry = match REGISTRY.lock() {
        Ok(registry) => registry,
        Err(poisoned) => poisoned.into_inner(),
    };

    registry
        .iter()
        .map(|(name, body)| format!("@keyframes {name} {{ {body} }}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    // These exercise the bookkeeping only. `register` reaches into the DOM, so
    // the injection itself is covered by the browser tests in `dwui`.

    #[test]
    fn css_renders_a_complete_rule() {
        let kf = Keyframes::new("test-fade", "from{opacity:0;}to{opacity:1;}");

        assert_eq!(
            kf.css(),
            "@keyframes test-fade { from{opacity:0;}to{opacity:1;} }"
        );
    }

    #[test]
    fn name_unregistered_does_not_register() {
        let kf = Keyframes::new("test-untouched", "from{opacity:0;}");

        assert_eq!(kf.name_unregistered(), "test-untouched");
        assert!(!is_registered("test-untouched"));
    }
}
