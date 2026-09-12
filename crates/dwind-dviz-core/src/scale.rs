//! Scales map data to pixels (or to colors).
//!
//! Continuous scales implement [`ContinuousScale`]; [`BandScale`] handles
//! categories; [`SequentialScale`] and [`DivergingScale`] map magnitude and
//! polarity to color through a [`Ramp`]. Every scale is a plain value, so
//! the renderer can rebuild one per frame from signals without allocation
//! beyond the band scale's category list.

use crate::data::Extent;
use crate::palette::{Color, Ramp};
use crate::ticks;
use jiff::Timestamp;
use jiff::tz::TimeZone;

/// A scale from a continuous input to a pixel position.
pub trait ContinuousScale {
    type Input: Copy;

    fn map(&self, v: Self::Input) -> f64;
    fn invert(&self, px: f64) -> Self::Input;
    fn range(&self) -> (f64, f64);
    /// Roughly `count` tick values inside the domain.
    fn ticks(&self, count: usize) -> Vec<Self::Input>;
}

fn lerp_range((r0, r1): (f64, f64), t: f64) -> f64 {
    r0 + (r1 - r0) * t
}

fn unlerp_range((r0, r1): (f64, f64), px: f64) -> f64 {
    if r1 == r0 { 0.0 } else { (px - r0) / (r1 - r0) }
}

/// The everyday scale: `domain.min` maps to `range.0`, `domain.max` to
/// `range.1`, linearly between.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearScale {
    pub domain: Extent<f64>,
    pub range: (f64, f64),
    /// Clamp outputs to the range instead of extrapolating.
    pub clamp: bool,
}

impl LinearScale {
    pub fn new(domain: Extent<f64>, range: (f64, f64)) -> Self {
        Self {
            domain,
            range,
            clamp: false,
        }
    }

    pub fn clamped(mut self) -> Self {
        self.clamp = true;
        self
    }

    /// Rounds the domain outward to tick boundaries for `count` ticks.
    pub fn nice(mut self, count: usize) -> Self {
        self.domain = ticks::nice_extent(self.domain, count);
        self
    }

    /// The step between ticks, for label formatting.
    pub fn tick_step(&self, count: usize) -> f64 {
        ticks::tick_step(self.domain.min, self.domain.max, count).abs()
    }
}

impl ContinuousScale for LinearScale {
    type Input = f64;

    fn map(&self, v: f64) -> f64 {
        let mut t = self.domain.normalize(v);
        if self.clamp {
            t = t.clamp(0.0, 1.0);
        }
        lerp_range(self.range, t)
    }

    fn invert(&self, px: f64) -> f64 {
        self.domain.lerp(unlerp_range(self.range, px))
    }

    fn range(&self) -> (f64, f64) {
        self.range
    }

    fn ticks(&self, count: usize) -> Vec<f64> {
        ticks::linear_ticks(self.domain.min, self.domain.max, count)
    }
}

/// A logarithmic scale. The domain must be strictly positive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogScale {
    pub domain: Extent<f64>,
    pub range: (f64, f64),
    pub base: f64,
    pub clamp: bool,
}

impl LogScale {
    /// Panics if the domain touches zero or is negative.
    pub fn new(domain: Extent<f64>, range: (f64, f64)) -> Self {
        assert!(
            domain.min > 0.0 && domain.max > 0.0,
            "log scale domain must be positive"
        );
        Self {
            domain,
            range,
            base: 10.0,
            clamp: false,
        }
    }

    pub fn with_base(mut self, base: f64) -> Self {
        self.base = base;
        self
    }

    pub fn clamped(mut self) -> Self {
        self.clamp = true;
        self
    }

    /// Rounds the domain outward to whole powers of the base.
    pub fn nice(mut self) -> Self {
        let lo = self.domain.min.log(self.base).floor();
        let hi = self.domain.max.log(self.base).ceil();
        self.domain = Extent::new(self.base.powf(lo), self.base.powf(hi));
        self
    }

    fn log_domain(&self) -> Extent<f64> {
        Extent::new(
            self.domain.min.log(self.base),
            self.domain.max.log(self.base),
        )
    }
}

impl ContinuousScale for LogScale {
    type Input = f64;

    fn map(&self, v: f64) -> f64 {
        let mut t = self.log_domain().normalize(v.log(self.base));
        if self.clamp {
            t = t.clamp(0.0, 1.0);
        }
        lerp_range(self.range, t)
    }

    fn invert(&self, px: f64) -> f64 {
        self.base
            .powf(self.log_domain().lerp(unlerp_range(self.range, px)))
    }

    fn range(&self) -> (f64, f64) {
        self.range
    }

    fn ticks(&self, count: usize) -> Vec<f64> {
        ticks::log_ticks(self.domain, self.base, count)
    }
}

/// A power scale; `exponent = 0.5` is the square-root scale used to size
/// bubbles by area rather than radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowScale {
    pub domain: Extent<f64>,
    pub range: (f64, f64),
    pub exponent: f64,
    pub clamp: bool,
}

impl PowScale {
    pub fn new(domain: Extent<f64>, range: (f64, f64), exponent: f64) -> Self {
        Self {
            domain,
            range,
            exponent,
            clamp: false,
        }
    }

    pub fn sqrt(domain: Extent<f64>, range: (f64, f64)) -> Self {
        Self::new(domain, range, 0.5)
    }

    pub fn clamped(mut self) -> Self {
        self.clamp = true;
        self
    }

    fn raise(&self, v: f64) -> f64 {
        v.signum() * v.abs().powf(self.exponent)
    }

    fn pow_domain(&self) -> Extent<f64> {
        Extent::new(self.raise(self.domain.min), self.raise(self.domain.max))
    }
}

impl ContinuousScale for PowScale {
    type Input = f64;

    fn map(&self, v: f64) -> f64 {
        let mut t = self.pow_domain().normalize(self.raise(v));
        if self.clamp {
            t = t.clamp(0.0, 1.0);
        }
        lerp_range(self.range, t)
    }

    fn invert(&self, px: f64) -> f64 {
        let r = self.pow_domain().lerp(unlerp_range(self.range, px));
        r.signum() * r.abs().powf(1.0 / self.exponent)
    }

    fn range(&self) -> (f64, f64) {
        self.range
    }

    fn ticks(&self, count: usize) -> Vec<f64> {
        ticks::linear_ticks(self.domain.min, self.domain.max, count)
    }
}

/// A time scale: linear in milliseconds, with calendar-aware ticks in a
/// time zone.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeScale {
    pub domain: Extent<Timestamp>,
    pub range: (f64, f64),
    pub tz: TimeZone,
    pub clamp: bool,
}

impl TimeScale {
    pub fn new(domain: Extent<Timestamp>, range: (f64, f64)) -> Self {
        Self {
            domain,
            range,
            tz: TimeZone::UTC,
            clamp: false,
        }
    }

    pub fn in_zone(mut self, tz: TimeZone) -> Self {
        self.tz = tz;
        self
    }

    pub fn clamped(mut self) -> Self {
        self.clamp = true;
        self
    }

    fn linear(&self) -> LinearScale {
        LinearScale {
            domain: self.domain.as_millis(),
            range: self.range,
            clamp: self.clamp,
        }
    }

    /// Maps a raw millisecond timestamp (what [`crate::data::TimePoint::to_point`]
    /// yields), avoiding a `Timestamp` round trip per point.
    pub fn map_millis(&self, ms: f64) -> f64 {
        self.linear().map(ms)
    }

    /// Ticks plus the interval that produced them, for label formatting.
    pub fn time_ticks(&self, count: usize) -> ticks::TimeTicks {
        ticks::time_ticks(self.domain, count, &self.tz)
    }
}

impl ContinuousScale for TimeScale {
    type Input = Timestamp;

    fn map(&self, v: Timestamp) -> f64 {
        self.map_millis(v.as_millisecond() as f64)
    }

    fn invert(&self, px: f64) -> Timestamp {
        let ms = self.linear().invert(px);
        Timestamp::from_millisecond(ms.round() as i64).unwrap_or(Timestamp::UNIX_EPOCH)
    }

    fn range(&self) -> (f64, f64) {
        self.range
    }

    fn ticks(&self, count: usize) -> Vec<Timestamp> {
        self.time_ticks(count).ticks
    }
}

/// Maps categories to evenly spaced bands (bars) or points (dot plots,
/// categorical x axes for lines).
///
/// With `padding_inner` and `padding_outer` in `0.0..=1.0` as fractions of
/// the step. `BandScale::points` sets the bandwidth to zero so each category
/// is a position.
#[derive(Debug, Clone, PartialEq)]
pub struct BandScale {
    categories: Vec<String>,
    pub range: (f64, f64),
    pub padding_inner: f64,
    pub padding_outer: f64,
    /// Where the leftover outer space goes: `0.0` all at the end, `0.5`
    /// centred, `1.0` all at the start.
    pub align: f64,
}

impl BandScale {
    pub fn new(categories: impl IntoIterator<Item = impl Into<String>>, range: (f64, f64)) -> Self {
        Self {
            categories: categories.into_iter().map(Into::into).collect(),
            range,
            padding_inner: 0.2,
            padding_outer: 0.1,
            align: 0.5,
        }
    }

    /// A point scale: zero bandwidth, categories at evenly spaced positions
    /// with `padding` of a step at each end.
    pub fn points(
        categories: impl IntoIterator<Item = impl Into<String>>,
        range: (f64, f64),
        padding: f64,
    ) -> Self {
        let mut s = Self::new(categories, range);
        s.padding_inner = 1.0;
        s.padding_outer = padding;
        s
    }

    pub fn with_padding(mut self, inner: f64, outer: f64) -> Self {
        self.padding_inner = inner.clamp(0.0, 1.0);
        self.padding_outer = outer.max(0.0);
        self
    }

    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    pub fn len(&self) -> usize {
        self.categories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    /// Distance between the starts of adjacent bands.
    pub fn step(&self) -> f64 {
        let n = self.categories.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        let span = (self.range.1 - self.range.0).abs();
        let denominator = n - self.padding_inner + 2.0 * self.padding_outer;
        if denominator <= 0.0 {
            0.0
        } else {
            span / denominator
        }
    }

    /// Width of each band; zero for a point scale.
    pub fn bandwidth(&self) -> f64 {
        self.step() * (1.0 - self.padding_inner)
    }

    /// Start position of the band at `index`.
    pub fn map_index(&self, index: usize) -> Option<f64> {
        if index >= self.categories.len() {
            return None;
        }
        let step = self.step();
        let n = self.categories.len() as f64;
        let span = (self.range.1 - self.range.0).abs();
        let used = step * (n - self.padding_inner);
        let start_offset = (span - used) * self.align;
        let offset = start_offset + step * index as f64;
        Some(if self.range.1 >= self.range.0 {
            self.range.0 + offset
        } else {
            self.range.0 - offset - self.bandwidth()
        })
    }

    /// Start position of the band for `category`.
    pub fn map(&self, category: &str) -> Option<f64> {
        self.index_of(category).and_then(|i| self.map_index(i))
    }

    /// Centre of the band for `category`; the position for a point scale.
    pub fn center(&self, category: &str) -> Option<f64> {
        self.map(category).map(|x| x + self.bandwidth() / 2.0)
    }

    pub fn index_of(&self, category: &str) -> Option<usize> {
        self.categories.iter().position(|c| c == category)
    }

    /// The category whose band (or nearest step) contains `px`, for hover.
    pub fn invert(&self, px: f64) -> Option<&str> {
        if self.categories.is_empty() {
            return None;
        }
        let step = self.step();
        if step <= 0.0 {
            return self.categories.first().map(String::as_str);
        }
        let first = self.map_index(0)?;
        let rel = if self.range.1 >= self.range.0 {
            px - first + (step - self.bandwidth()) / 2.0
        } else {
            first + self.bandwidth() - px + (step - self.bandwidth()) / 2.0
        };
        let i = (rel / step).floor();
        let i = i.clamp(0.0, (self.categories.len() - 1) as f64) as usize;
        self.categories.get(i).map(String::as_str)
    }
}

/// Maps a magnitude to a color along a one-hue ramp.
#[derive(Debug, Clone, PartialEq)]
pub struct SequentialScale {
    pub domain: Extent<f64>,
    pub ramp: Ramp,
}

impl SequentialScale {
    pub fn new(domain: Extent<f64>, ramp: Ramp) -> Self {
        Self { domain, ramp }
    }

    pub fn map(&self, v: f64) -> Color {
        self.ramp.at(self.domain.normalize(v).clamp(0.0, 1.0))
    }

    /// `n` legend swatches with their threshold values.
    pub fn legend(&self, n: usize) -> Vec<(f64, Color)> {
        (0..n)
            .map(|i| {
                let t = if n <= 1 {
                    0.5
                } else {
                    i as f64 / (n - 1) as f64
                };
                (self.domain.lerp(t), self.ramp.at(t))
            })
            .collect()
    }
}

/// Maps a signed quantity to a two-hue ramp through a neutral midpoint.
/// The two arms are scaled separately so `midpoint` always lands on the
/// neutral stop even when the domain is asymmetric.
#[derive(Debug, Clone, PartialEq)]
pub struct DivergingScale {
    pub domain: Extent<f64>,
    pub midpoint: f64,
    pub ramp: Ramp,
}

impl DivergingScale {
    pub fn new(domain: Extent<f64>, midpoint: f64, ramp: Ramp) -> Self {
        Self {
            domain,
            midpoint,
            ramp,
        }
    }

    /// A domain symmetric about `midpoint`, so equal magnitudes on either
    /// side get equally strong colors. Usually what you want.
    pub fn symmetric(extent: Extent<f64>, midpoint: f64, ramp: Ramp) -> Self {
        let reach = (extent.max - midpoint)
            .abs()
            .max((extent.min - midpoint).abs());
        Self::new(
            Extent::new(midpoint - reach, midpoint + reach),
            midpoint,
            ramp,
        )
    }

    pub fn map(&self, v: f64) -> Color {
        let t = if v < self.midpoint {
            let arm = (self.midpoint - self.domain.min).abs();
            if arm == 0.0 {
                0.5
            } else {
                0.5 - 0.5 * ((self.midpoint - v) / arm).clamp(0.0, 1.0)
            }
        } else {
            let arm = (self.domain.max - self.midpoint).abs();
            if arm == 0.0 {
                0.5
            } else {
                0.5 + 0.5 * ((v - self.midpoint) / arm).clamp(0.0, 1.0)
            }
        };
        self.ramp.at(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::defaults;

    #[test]
    fn linear_maps_and_inverts() {
        let s = LinearScale::new(Extent::new(0.0, 100.0), (300.0, 0.0));
        assert_eq!(s.map(0.0), 300.0);
        assert_eq!(s.map(100.0), 0.0);
        assert_eq!(s.map(25.0), 225.0);
        assert_eq!(s.invert(225.0), 25.0);
        assert_eq!(s.map(150.0), -150.0, "extrapolates by default");
        assert_eq!(s.clamped().map(150.0), 0.0);
        assert_eq!(s.nice(5).domain, Extent::new(0.0, 100.0));
        assert_eq!(
            LinearScale::new(Extent::new(3.0, 97.0), (0.0, 1.0))
                .nice(10)
                .domain,
            Extent::new(0.0, 100.0)
        );
    }

    #[test]
    fn degenerate_linear_domain_maps_to_middle() {
        let s = LinearScale::new(Extent::new(5.0, 5.0), (0.0, 100.0));
        assert_eq!(s.map(5.0), 50.0);
        assert!(s.ticks(5) == vec![5.0]);
    }

    #[test]
    fn log_scale() {
        let s = LogScale::new(Extent::new(1.0, 1000.0), (0.0, 300.0));
        assert_eq!(s.map(1.0), 0.0);
        assert!((s.map(10.0) - 100.0).abs() < 1e-9);
        assert!((s.invert(200.0) - 100.0).abs() < 1e-9);
        assert_eq!(
            LogScale::new(Extent::new(3.0, 800.0), (0.0, 1.0))
                .nice()
                .domain,
            Extent::new(1.0, 1000.0)
        );
        assert_eq!(s.ticks(3), vec![1.0, 10.0, 100.0, 1000.0]);
    }

    #[test]
    fn sqrt_scale_sizes_by_area() {
        let s = PowScale::sqrt(Extent::new(0.0, 100.0), (0.0, 10.0));
        assert_eq!(s.map(25.0), 5.0);
        assert!((s.invert(5.0) - 25.0).abs() < 1e-9);
    }

    #[test]
    fn time_scale_maps_and_ticks() {
        let a: Timestamp = "2026-01-01T00:00:00Z".parse().unwrap();
        let b: Timestamp = "2026-01-01T01:00:00Z".parse().unwrap();
        let s = TimeScale::new(Extent::new(a, b), (0.0, 600.0));
        let half: Timestamp = "2026-01-01T00:30:00Z".parse().unwrap();
        assert_eq!(s.map(half), 300.0);
        assert_eq!(s.invert(300.0), half);
        let t = s.time_ticks(4);
        assert_eq!(t.interval, ticks::TimeInterval::Minute(15));
        assert_eq!(t.ticks.len(), 5);
    }

    #[test]
    fn band_scale_geometry() {
        let s = BandScale::new(["a", "b", "c"], (0.0, 100.0)).with_padding(0.0, 0.0);
        assert!((s.step() - 100.0 / 3.0).abs() < 1e-9);
        assert_eq!(s.bandwidth(), s.step());
        assert_eq!(s.map("a"), Some(0.0));
        assert!((s.map("c").unwrap() - 200.0 / 3.0).abs() < 1e-9);
        assert_eq!(s.map("zzz"), None);

        let padded = BandScale::new(["a", "b"], (0.0, 100.0)).with_padding(0.5, 0.25);
        // step = 100 / (2 - 0.5 + 0.5) = 50; bandwidth = 25.
        assert_eq!(padded.step(), 50.0);
        assert_eq!(padded.bandwidth(), 25.0);
        assert_eq!(padded.map("a"), Some(12.5));
        assert_eq!(padded.center("b"), Some(75.0));
    }

    #[test]
    fn band_scale_flipped_range_and_invert() {
        let s = BandScale::new(["a", "b", "c"], (100.0, 0.0)).with_padding(0.0, 0.0);
        let a = s.map("a").unwrap();
        assert!(
            (a - 200.0 / 3.0).abs() < 1e-9,
            "first band sits at the far end: {a}"
        );
        assert_eq!(s.invert(90.0), Some("a"));
        assert_eq!(s.invert(5.0), Some("c"));

        let fwd = BandScale::new(["a", "b", "c"], (0.0, 300.0));
        assert_eq!(fwd.invert(-50.0), Some("a"));
        assert_eq!(fwd.invert(150.0), Some("b"));
        assert_eq!(fwd.invert(1000.0), Some("c"));
    }

    #[test]
    fn point_scale_has_no_width() {
        let s = BandScale::points(["a", "b", "c"], (0.0, 100.0), 0.5);
        assert_eq!(s.bandwidth(), 0.0);
        assert!((s.center("a").unwrap() - 100.0 / 6.0).abs() < 1e-9);
        assert!((s.center("c").unwrap() - 500.0 / 6.0).abs() < 1e-9);
        assert_eq!(
            BandScale::points(["a"], (0.0, 100.0), 0.5).center("a"),
            Some(50.0)
        );
    }

    #[test]
    fn color_scales() {
        let seq = SequentialScale::new(
            Extent::new(0.0, 10.0),
            defaults::sequential(crate::palette::Mode::Light),
        );
        assert_eq!(seq.map(-5.0), defaults::SEQUENTIAL_BLUE[0]);
        assert_eq!(seq.map(10.0), defaults::SEQUENTIAL_BLUE[12]);
        assert_eq!(seq.legend(3).len(), 3);

        let div = DivergingScale::symmetric(
            Extent::new(-2.0, 8.0),
            0.0,
            defaults::diverging(crate::palette::Mode::Light),
        );
        assert_eq!(div.domain, Extent::new(-8.0, 8.0));
        assert_eq!(div.map(0.0), defaults::DIVERGING_MID_LIGHT);
        assert_eq!(div.map(8.0), defaults::DIVERGING_POS);
        assert_eq!(div.map(-100.0), defaults::DIVERGING_NEG);
        let asym = DivergingScale::new(
            Extent::new(-2.0, 8.0),
            0.0,
            defaults::diverging(crate::palette::Mode::Light),
        );
        assert_eq!(
            asym.map(-2.0),
            defaults::DIVERGING_NEG,
            "each arm scaled on its own"
        );
    }
}
