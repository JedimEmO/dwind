//! Which series are hidden, toggled from the legend.
//!
//! Hiding filters the series a chart draws; the survivors keep their color
//! slots because slots follow ids, never positions.

use std::collections::BTreeSet;
use std::rc::Rc;

use dwind_dviz_core::data::Series;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

#[derive(Debug, Default)]
pub struct SeriesVisibility {
    hidden: Mutable<BTreeSet<String>>,
}

impl SeriesVisibility {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    pub fn toggle(&self, id: &str) {
        let mut hidden = self.hidden.lock_mut();
        if !hidden.remove(id) {
            hidden.insert(id.to_string());
        }
    }

    pub fn set_hidden(&self, id: &str, hidden: bool) {
        let mut set = self.hidden.lock_mut();
        if hidden {
            set.insert(id.to_string());
        } else {
            set.remove(id);
        }
    }

    pub fn is_hidden(&self, id: &str) -> bool {
        self.hidden.lock_ref().contains(id)
    }

    pub fn hidden_signal(&self) -> impl Signal<Item = BTreeSet<String>> + use<> {
        self.hidden.signal_cloned()
    }

    /// `series` with the hidden ones removed.
    pub fn filter<S>(&self, series: S) -> impl Signal<Item = Vec<Series>> + use<S>
    where
        S: Signal<Item = Vec<Series>>,
    {
        map_ref! {
            let all = series,
            let hidden = self.hidden.signal_cloned() =>
            all.iter().filter(|s| !hidden.contains(&s.id)).cloned().collect::<Vec<_>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_and_query() {
        let v = SeriesVisibility::new();
        assert!(!v.is_hidden("a"));
        v.toggle("a");
        assert!(v.is_hidden("a"));
        v.toggle("a");
        assert!(!v.is_hidden("a"));
        v.set_hidden("b", true);
        assert!(v.is_hidden("b"));
    }
}
