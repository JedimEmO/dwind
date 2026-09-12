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

use dwind_dviz_core::data::{DataError, Extent, Series, validate_series, x_extent_of};
use futures_signals::signal::{Mutable, Signal, SignalExt, always};

/// A boxed series signal, for sources handed around as trait objects.
pub type BoxedSeriesSignal = std::pin::Pin<Box<dyn Signal<Item = Vec<Series>>>>;
pub type BoxedExtentSignal = std::pin::Pin<Box<dyn Signal<Item = Option<Extent<f64>>>>>;

/// Anything that yields series as a signal.
pub trait DataSource {
    fn series_signal(&self) -> BoxedSeriesSignal;
}

/// A data source that can provide an explicit x-domain for fixed or live
/// charts. The domain is optional because an empty source has no extent.
pub trait DomainSource: DataSource {
    fn x_domain_signal(&self) -> BoxedExtentSignal;
}

/// Pre-prepared data that never changes.
#[derive(Debug, Clone, PartialEq)]
pub struct StaticSource(pub Vec<Series>);

impl StaticSource {
    /// Creates a source after validating series identities and coordinates.
    pub fn try_new(series: Vec<Series>) -> Result<Self, DataError> {
        validate_series(&series)?;
        Ok(Self(series))
    }
}

impl DataSource for StaticSource {
    fn series_signal(&self) -> BoxedSeriesSignal {
        Box::pin(always(self.0.clone()))
    }
}

impl DomainSource for StaticSource {
    fn x_domain_signal(&self) -> BoxedExtentSignal {
        Box::pin(always(x_extent_of(&self.0)))
    }
}

/// Data the application owns and replaces wholesale.
#[derive(Debug, Clone, Default)]
pub struct MutableSource(pub Mutable<Vec<Series>>);

impl MutableSource {
    /// Creates a mutable source after validating its initial snapshot.
    pub fn try_new(series: Vec<Series>) -> Result<Self, DataError> {
        validate_series(&series)?;
        Ok(Self::new(series))
    }

    pub fn new(series: Vec<Series>) -> Self {
        Self(Mutable::new(series))
    }

    pub fn set(&self, series: Vec<Series>) {
        self.0.set(series);
    }

    /// Replaces the snapshot only when the complete update is valid.
    pub fn try_set(&self, series: Vec<Series>) -> Result<(), DataError> {
        validate_series(&series)?;
        self.set(series);
        Ok(())
    }
}

impl DataSource for MutableSource {
    fn series_signal(&self) -> BoxedSeriesSignal {
        Box::pin(self.0.signal_cloned())
    }
}

impl DomainSource for MutableSource {
    fn x_domain_signal(&self) -> BoxedExtentSignal {
        Box::pin(self.0.signal_cloned().map(|series| x_extent_of(&series)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dwind_dviz_core::data::Point;

    fn series(id: &str) -> Series {
        Series::new(id, id, vec![Point::new(0.0, 1.0)])
    }

    #[test]
    fn source_constructors_validate_complete_snapshots() {
        let duplicate = vec![series("same"), series("same")];
        assert!(StaticSource::try_new(duplicate.clone()).is_err());
        assert!(MutableSource::try_new(duplicate).is_err());
    }

    #[test]
    fn mutable_try_set_preserves_the_previous_valid_snapshot() {
        let source = MutableSource::try_new(vec![series("good")]).unwrap();
        let invalid = vec![series("same"), series("same")];
        assert!(source.try_set(invalid).is_err());
        assert_eq!(source.0.get_cloned(), vec![series("good")]);
    }
}
