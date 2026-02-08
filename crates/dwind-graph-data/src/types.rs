/// A 2D data point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

impl DataPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A named data series with a color index for palette mapping.
#[derive(Debug, Clone)]
pub struct Series {
    pub label: String,
    pub data: Vec<DataPoint>,
    pub color_index: usize,
}

impl Series {
    pub fn new(label: impl Into<String>, data: Vec<DataPoint>, color_index: usize) -> Self {
        Self {
            label: label.into(),
            data,
            color_index,
        }
    }

    pub fn x_extent(&self) -> Option<Extent> {
        Extent::from_iter(self.data.iter().map(|p| p.x))
    }

    pub fn y_extent(&self) -> Option<Extent> {
        Extent::from_iter(self.data.iter().map(|p| p.y))
    }
}

/// A slice in a pie/donut chart.
#[derive(Debug, Clone)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
    pub color_index: usize,
}

impl PieSlice {
    pub fn new(label: impl Into<String>, value: f64, color_index: usize) -> Self {
        Self {
            label: label.into(),
            value,
            color_index,
        }
    }
}

/// A numeric range with min and max.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extent {
    pub min: f64,
    pub max: f64,
}

impl Extent {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Compute extent from an iterator of values.
    pub fn from_iter(values: impl Iterator<Item = f64>) -> Option<Self> {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut any = false;
        for v in values {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
            any = true;
        }
        if any {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// Combine two extents into one that covers both.
    pub fn union(&self, other: &Extent) -> Extent {
        Extent {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Compute the combined extent across multiple series (y values).
    pub fn y_extent_of_series(series: &[Series]) -> Option<Extent> {
        series
            .iter()
            .filter_map(|s| s.y_extent())
            .reduce(|a, b| a.union(&b))
    }

    /// Compute the combined extent across multiple series (x values).
    pub fn x_extent_of_series(series: &[Series]) -> Option<Extent> {
        series
            .iter()
            .filter_map(|s| s.x_extent())
            .reduce(|a, b| a.union(&b))
    }

    /// Expand the extent by a factor on each side. E.g. `pad(0.05)` adds 5% padding.
    pub fn pad(&self, factor: f64) -> Self {
        let range = self.max - self.min;
        if range == 0.0 {
            // Avoid zero-range: expand by 1.0 in each direction
            return Self {
                min: self.min - 1.0,
                max: self.max + 1.0,
            };
        }
        let padding = range * factor;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// Round to "nice" boundaries (multiples of 1, 2, 5, 10, ...).
    pub fn nice(&self) -> Self {
        let range = self.max - self.min;
        if range == 0.0 {
            return self.pad(0.1);
        }
        let step = nice_step(range, 10);
        Self {
            min: (self.min / step).floor() * step,
            max: (self.max / step).ceil() * step,
        }
    }

    pub fn range(&self) -> f64 {
        self.max - self.min
    }
}

/// Find a "nice" step size for the given range and target tick count.
pub(crate) fn nice_step(range: f64, target_count: usize) -> f64 {
    let rough_step = range / target_count as f64;
    let magnitude = 10.0_f64.powf(rough_step.log10().floor());
    let residual = rough_step / magnitude;

    let nice = if residual <= 1.5 {
        1.0
    } else if residual <= 3.5 {
        2.0
    } else if residual <= 7.5 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extent_from_iter() {
        let ext = Extent::from_iter([1.0, 5.0, 3.0, -2.0, 8.0].into_iter()).unwrap();
        assert_eq!(ext.min, -2.0);
        assert_eq!(ext.max, 8.0);
    }

    #[test]
    fn test_extent_from_empty() {
        assert!(Extent::from_iter(std::iter::empty()).is_none());
    }

    #[test]
    fn test_extent_pad() {
        let ext = Extent::new(0.0, 100.0).pad(0.05);
        assert!((ext.min - (-5.0)).abs() < 1e-10);
        assert!((ext.max - 105.0).abs() < 1e-10);
    }

    #[test]
    fn test_extent_nice() {
        let ext = Extent::new(0.3, 9.7).nice();
        assert_eq!(ext.min, 0.0);
        assert_eq!(ext.max, 10.0);
    }

    #[test]
    fn test_extent_union() {
        let a = Extent::new(0.0, 5.0);
        let b = Extent::new(-2.0, 3.0);
        let u = a.union(&b);
        assert_eq!(u.min, -2.0);
        assert_eq!(u.max, 5.0);
    }

    #[test]
    fn test_nice_step() {
        assert!((nice_step(100.0, 10) - 10.0).abs() < 1e-10);
        assert!((nice_step(45.0, 10) - 5.0).abs() < 1e-10);
        assert!((nice_step(0.8, 10) - 0.1).abs() < 1e-10);
    }
}
