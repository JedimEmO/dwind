//! The data model: points, series, and extents.
//!
//! Deliberately small. Pixel-space layout, colors and interaction live
//! elsewhere; this module only describes *what* is plotted.

use jiff::Timestamp;
use thiserror::Error;

/// Validation failures for data handed to a chart.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum DataError {
    #[error("series id must not be empty")]
    EmptySeriesId,
    #[error("series id is duplicated: {0}")]
    DuplicateSeriesId(String),
    #[error("series {series:?} contains a non-finite x value at point {index}")]
    NonFiniteX { series: String, index: usize },
    #[error("series {series:?} contains an infinite y value at point {index}")]
    InfiniteY { series: String, index: usize },
    #[error("category is not present in the domain: {0}")]
    UnknownCategory(String),
}

/// A closed interval `[min, max]`. The building block of every continuous
/// domain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extent<T> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd + Copy> Extent<T> {
    /// Builds an extent, swapping the bounds if they were given reversed.
    pub fn new(a: T, b: T) -> Self {
        if a <= b {
            Self { min: a, max: b }
        } else {
            Self { min: b, max: a }
        }
    }

    pub fn contains(&self, v: T) -> bool {
        self.min <= v && v <= self.max
    }

    /// The smallest extent covering both.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: if other.min < self.min {
                other.min
            } else {
                self.min
            },
            max: if other.max > self.max {
                other.max
            } else {
                self.max
            },
        }
    }

    /// Grows the extent to include `v`.
    pub fn include(&self, v: T) -> Self {
        Self {
            min: if v < self.min { v } else { self.min },
            max: if v > self.max { v } else { self.max },
        }
    }
}

impl Extent<f64> {
    pub const UNIT: Self = Self { min: 0.0, max: 1.0 };

    pub fn span(&self) -> f64 {
        self.max - self.min
    }

    /// True when `min == max` (or the extent is not finite), which every
    /// scale must guard against before dividing by the span.
    pub fn is_degenerate(&self) -> bool {
        !(self.span().is_finite() && self.span() > 0.0)
    }

    /// Pads both ends by `fraction` of the span. A degenerate extent is
    /// padded by `fraction` of `|min|` (or by `1.0` at zero) so it opens up.
    pub fn pad(&self, fraction: f64) -> Self {
        let fraction = fraction.max(0.0);
        if self.is_degenerate() {
            let amount = if self.min == 0.0 {
                1.0
            } else {
                self.min.abs() * fraction
            };
            return Self {
                min: self.min - amount,
                max: self.max + amount,
            };
        }
        let amount = self.span() * fraction;
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    /// Ensures zero is inside the extent. Bar charts always call this: a bar
    /// that does not grow from zero misrepresents its value.
    pub fn include_zero(&self) -> Self {
        self.include(0.0)
    }

    /// Position of `v` inside the extent as a fraction, `0.0` at `min` and
    /// `1.0` at `max`. Not clamped.
    pub fn normalize(&self, v: f64) -> f64 {
        if self.is_degenerate() {
            0.5
        } else {
            (v - self.min) / self.span()
        }
    }

    pub fn lerp(&self, t: f64) -> f64 {
        self.min + t * self.span()
    }
}

impl Extent<Timestamp> {
    pub fn span_ms(&self) -> f64 {
        (self.max.as_millisecond() - self.min.as_millisecond()) as f64
    }

    /// The same extent in floating-point milliseconds since the Unix epoch,
    /// which is what a continuous scale works in.
    pub fn as_millis(&self) -> Extent<f64> {
        Extent {
            min: self.min.as_millisecond() as f64,
            max: self.max.as_millisecond() as f64,
        }
    }
}

/// A single `(x, y)` observation on two continuous axes.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// A point is plotted only when both coordinates are finite; a NaN `y`
    /// is the conventional way to punch a gap into a line.
    pub fn is_defined(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

/// A timestamped observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimePoint {
    pub t: Timestamp,
    pub y: f64,
}

impl TimePoint {
    pub const fn new(t: Timestamp, y: f64) -> Self {
        Self { t, y }
    }

    /// Projects onto the millisecond axis a `TimeScale` works in.
    pub fn to_point(&self) -> Point {
        Point {
            x: self.t.as_millisecond() as f64,
            y: self.y,
        }
    }
}

/// A category / value pair for band charts.
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryPoint {
    pub category: String,
    pub value: f64,
}

impl CategoryPoint {
    pub fn new(category: impl Into<String>, value: f64) -> Self {
        Self {
            category: category.into(),
            value,
        }
    }

    /// Resolves this category against a declared band domain.
    pub fn to_point(&self, domain: &[String]) -> Result<Point, DataError> {
        let index = domain
            .iter()
            .position(|category| category == &self.category)
            .ok_or_else(|| DataError::UnknownCategory(self.category.clone()))?;
        Ok(Point::new(index as f64, self.value))
    }
}

/// A named sequence of observations.
///
/// `id` is the stable identity used for color assignment and keyed DOM
/// patching; it must not change when the series is filtered or reordered.
/// `label` is what people read in legends and tooltips.
#[derive(Debug, Clone, PartialEq)]
pub struct Series<P = Point> {
    pub id: String,
    pub label: String,
    pub points: Vec<P>,
}

impl<P> Series<P> {
    pub fn new(id: impl Into<String>, label: impl Into<String>, points: Vec<P>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            points,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

impl Series<Point> {
    /// Constructs a validated numeric series. NaN y values are retained as
    /// intentional line gaps; infinite coordinates are rejected.
    pub fn try_new(
        id: impl Into<String>,
        label: impl Into<String>,
        points: Vec<Point>,
    ) -> Result<Self, DataError> {
        let series = Self::new(id, label, points);
        series.validate()?;
        Ok(series)
    }

    pub fn validate(&self) -> Result<(), DataError> {
        if self.id.is_empty() {
            return Err(DataError::EmptySeriesId);
        }
        for (index, point) in self.points.iter().enumerate() {
            if !point.x.is_finite() {
                return Err(DataError::NonFiniteX {
                    series: self.id.clone(),
                    index,
                });
            }
            if point.y.is_infinite() {
                return Err(DataError::InfiniteY {
                    series: self.id.clone(),
                    index,
                });
            }
        }
        Ok(())
    }

    /// Extents of the defined points, or `None` when there are none.
    pub fn x_extent(&self) -> Option<Extent<f64>> {
        crate::stats::extent(self.points.iter().filter(|p| p.is_defined()).map(|p| p.x))
    }

    pub fn y_extent(&self) -> Option<Extent<f64>> {
        crate::stats::extent(self.points.iter().filter(|p| p.is_defined()).map(|p| p.y))
    }
}

impl Series<CategoryPoint> {
    /// Converts typed category/value observations into the numeric point
    /// representation consumed by the SVG renderer.
    pub fn into_numeric(self, domain: &[String]) -> Result<Series<Point>, DataError> {
        let points = self
            .points
            .iter()
            .map(|point| point.to_point(domain))
            .collect::<Result<Vec<_>, _>>()?;
        Series::<Point>::try_new(self.id, self.label, points)
    }
}

/// Validates identities and numeric coordinates for a collection of series.
pub fn validate_series(series: &[Series]) -> Result<(), DataError> {
    let mut ids = std::collections::HashSet::with_capacity(series.len());
    for item in series {
        item.validate()?;
        if !ids.insert(item.id.clone()) {
            return Err(DataError::DuplicateSeriesId(item.id.clone()));
        }
    }
    Ok(())
}

/// Union of the y-extents of several series.
pub fn y_extent_of(series: &[Series]) -> Option<Extent<f64>> {
    series
        .iter()
        .filter_map(Series::y_extent)
        .reduce(|a, b| a.union(&b))
}

/// Union of the x-extents of several series.
pub fn x_extent_of(series: &[Series]) -> Option<Extent<f64>> {
    series
        .iter()
        .filter_map(Series::x_extent)
        .reduce(|a, b| a.union(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extent_new_orders_bounds() {
        let e = Extent::new(5.0, 1.0);
        assert_eq!(e, Extent { min: 1.0, max: 5.0 });
    }

    #[test]
    fn extent_pad_and_zero() {
        let e = Extent::new(10.0, 20.0).pad(0.1);
        assert_eq!(
            e,
            Extent {
                min: 9.0,
                max: 21.0
            }
        );
        assert_eq!(Extent::new(10.0, 20.0).include_zero().min, 0.0);
        assert_eq!(Extent::new(-3.0, -1.0).include_zero().max, 0.0);
    }

    #[test]
    fn degenerate_extent_opens_up() {
        let e = Extent::new(5.0, 5.0);
        assert!(e.is_degenerate());
        let p = e.pad(0.1);
        assert!(p.min < 5.0 && p.max > 5.0);
        assert!(!p.is_degenerate());
        assert_eq!(Extent::new(0.0, 0.0).pad(0.1), Extent::new(-1.0, 1.0));
        assert_eq!(e.normalize(5.0), 0.5);
        assert_eq!(Extent::new(10.0, 20.0).pad(-1.0), Extent::new(10.0, 20.0));
    }

    #[test]
    fn series_extents_skip_gaps() {
        let s = Series::new(
            "a",
            "A",
            vec![
                Point::new(0.0, 1.0),
                Point::new(1.0, f64::NAN),
                Point::new(2.0, 3.0),
            ],
        );
        assert_eq!(s.y_extent(), Some(Extent::new(1.0, 3.0)));
        assert_eq!(s.x_extent(), Some(Extent::new(0.0, 2.0)));
        let empty: Series = Series::new("e", "E", vec![]);
        assert_eq!(empty.y_extent(), None);
    }

    #[test]
    fn validated_series_rejects_bad_identity_and_coordinates() {
        assert_eq!(
            Series::try_new("", "Empty", vec![]),
            Err(DataError::EmptySeriesId)
        );
        assert_eq!(
            Series::try_new("a", "A", vec![Point::new(f64::INFINITY, 1.0)]),
            Err(DataError::NonFiniteX {
                series: "a".into(),
                index: 0,
            })
        );
        assert_eq!(
            Series::try_new("a", "A", vec![Point::new(0.0, f64::INFINITY)]),
            Err(DataError::InfiniteY {
                series: "a".into(),
                index: 0,
            })
        );
        assert!(Series::try_new("gap", "Gap", vec![Point::new(0.0, f64::NAN)]).is_ok());
    }

    #[test]
    fn validation_rejects_duplicate_series_ids() {
        let series = vec![
            Series::new("same", "A", vec![]),
            Series::new("same", "B", vec![]),
        ];
        assert_eq!(
            validate_series(&series),
            Err(DataError::DuplicateSeriesId("same".into()))
        );
    }

    #[test]
    fn category_points_require_a_declared_domain() {
        let domain = vec!["low".into(), "high".into()];
        let typed = Series::new(
            "priority",
            "Priority",
            vec![CategoryPoint::new("high", 3.0)],
        );
        let numeric = typed.into_numeric(&domain).unwrap();
        assert_eq!(numeric.points, vec![Point::new(1.0, 3.0)]);

        let unknown = Series::new(
            "priority",
            "Priority",
            vec![CategoryPoint::new("urgent", 5.0)],
        );
        assert_eq!(
            unknown.into_numeric(&domain),
            Err(DataError::UnknownCategory("urgent".into()))
        );
    }

    #[test]
    fn time_extent_in_millis() {
        let a = Timestamp::from_millisecond(1_000).unwrap();
        let b = Timestamp::from_millisecond(4_000).unwrap();
        let e = Extent::new(b, a);
        assert_eq!(e.span_ms(), 3_000.0);
        assert_eq!(e.as_millis(), Extent::new(1_000.0, 4_000.0));
    }
}
