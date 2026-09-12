//! Minimal-diff keyed lists.
//!
//! `Signal<Vec<K>>::to_signal_vec()` replaces the whole list on every
//! change, which tears down and recreates every child. [`diffed`] keeps a
//! `MutableVec` in sync with a key signal by removing, inserting and moving
//! only the keys that changed, so untouched nodes keep their DOM, their
//! state, and their in-flight transitions.

use std::future::Future;

use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;

/// A `MutableVec` that follows `keys` with minimal diffs, plus the future
/// that drives it. Attach the future to the node owning the children.
pub(crate) fn diffed<K, S>(keys: S) -> (MutableVec<K>, impl Future<Output = ()> + use<K, S>)
where
    K: Clone + Eq + 'static,
    S: Signal<Item = Vec<K>> + 'static,
{
    let vec: MutableVec<K> = MutableVec::new();
    let driver = keys.for_each({
        let vec = vec.clone();
        move |next| {
            apply(&vec, &next);
            async {}
        }
    });
    (vec, driver)
}

/// Brings `vec` to `next` in place.
pub(crate) fn apply<K: Clone + Eq>(vec: &MutableVec<K>, next: &[K]) {
    let mut lock = vec.lock_mut();
    // Remove what is gone, from the back so indices stay valid.
    for i in (0..lock.len()).rev() {
        if !next.contains(&lock[i]) {
            lock.remove(i);
        }
    }
    // Insert or move so position j holds next[j].
    for (j, key) in next.iter().enumerate() {
        if lock.get(j) == Some(key) {
            continue;
        }
        if let Some(at) = lock.iter().position(|k| k == key) {
            lock.move_from_to(at, j);
        } else {
            lock.insert_cloned(j, key.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(v: &MutableVec<&'static str>) -> Vec<&'static str> {
        v.lock_ref().to_vec()
    }

    #[test]
    fn applies_removals_insertions_and_moves() {
        let v: MutableVec<&'static str> = MutableVec::new();
        apply(&v, &["a", "b", "c"]);
        assert_eq!(snapshot(&v), vec!["a", "b", "c"]);
        apply(&v, &["a", "c"]);
        assert_eq!(snapshot(&v), vec!["a", "c"]);
        apply(&v, &["c", "a", "d"]);
        assert_eq!(snapshot(&v), vec!["c", "a", "d"]);
        apply(&v, &[]);
        assert!(snapshot(&v).is_empty());
    }
}
