//! Data sources for dwind-dviz.
//!
//! Bridges plain data and `futures-signals`. A chart takes any
//! `Signal<Item = Vec<Series>>`; this crate provides the ones that are
//! awkward to write by hand:
//!
//! - [`WindowedSource`]: ring buffers per series with count or time-span
//!   retention, a sliding x window, follow/pause, and batched commits so a
//!   burst of pushes costs one publish. The realtime primitive.
//! - [`drive`]: feeds a `Stream` of samples into a windowed source.
//! - [`downsample`]: caps the points per series to what the plot width can
//!   show, with min/max bucketing (keeps spikes) or LTTB (keeps shape).
//!
//! DOM-free: the renderer crate adds the `requestAnimationFrame` coalescing
//! on top through [`WindowedSource::set_notifier`].

pub mod downsample;
pub mod windowed;

pub use downsample::{Strategy, downsample};
pub use windowed::{Retention, WindowedSource, drive};

use dwind_dviz_core::data::Series;
use futures_signals::signal::{Mutable, Signal, always};

/// A boxed series signal, for sources handed around as trait objects.
pub type BoxedSeriesSignal = std::pin::Pin<Box<dyn Signal<Item = Vec<Series>>>>;

/// Anything that yields series as a signal.
pub trait DataSource {
    fn series_signal(&self) -> BoxedSeriesSignal;
}

/// Pre-prepared data that never changes.
#[derive(Debug, Clone, PartialEq)]
pub struct StaticSource(pub Vec<Series>);

impl DataSource for StaticSource {
    fn series_signal(&self) -> BoxedSeriesSignal {
        Box::pin(always(self.0.clone()))
    }
}

/// Data the application owns and replaces wholesale.
#[derive(Debug, Clone, Default)]
pub struct MutableSource(pub Mutable<Vec<Series>>);

impl MutableSource {
    pub fn new(series: Vec<Series>) -> Self {
        Self(Mutable::new(series))
    }

    pub fn set(&self, series: Vec<Series>) {
        self.0.set(series);
    }
}

impl DataSource for MutableSource {
    fn series_signal(&self) -> BoxedSeriesSignal {
        Box::pin(self.0.signal_cloned())
    }
}
