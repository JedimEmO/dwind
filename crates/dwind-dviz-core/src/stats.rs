//! Descriptive statistics and downsampling.
//!
//! Every function ignores non-finite values rather than propagating NaN, so a
//! gap in a series never poisons an axis domain.

use crate::data::{Extent, Point};

/// Minimum and maximum of the finite values, or `None` if there are none.
pub fn extent(values: impl IntoIterator<Item = f64>) -> Option<Extent<f64>> {
    values
        .into_iter()
        .filter(|v| v.is_finite())
        .fold(None, |acc: Option<Extent<f64>>, v| match acc {
            None => Some(Extent { min: v, max: v }),
            Some(e) => Some(e.include(v)),
        })
}

pub fn sum(values: impl IntoIterator<Item = f64>) -> f64 {
    values.into_iter().filter(|v| v.is_finite()).sum()
}

pub fn mean(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    let (n, total) = values
        .into_iter()
        .filter(|v| v.is_finite())
        .fold((0usize, 0.0), |(n, t), v| (n + 1, t + v));
    (n > 0).then(|| total / n as f64)
}

/// The `p`-quantile (`0.0..=1.0`) of `sorted`, linearly interpolated
/// (R-7 / the d3 convention). `sorted` must be ascending and finite.
pub fn quantile_sorted(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let p = p.clamp(0.0, 1.0);
    let pos = (sorted.len() - 1) as f64 * p;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    let frac = pos - lo as f64;
    Some(sorted[lo] + (sorted[hi] - sorted[lo]) * frac)
}

/// Sorts the finite values and returns the `p`-quantile.
pub fn quantile(values: impl IntoIterator<Item = f64>, p: f64) -> Option<f64> {
    let mut v: Vec<f64> = values.into_iter().filter(|v| v.is_finite()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    quantile_sorted(&v, p)
}

pub fn median(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    quantile(values, 0.5)
}

/// A histogram bucket over `[x0, x1)`; the last bucket is closed on the right.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bin {
    pub x0: f64,
    pub x1: f64,
    pub count: usize,
}

/// Buckets `values` into `count` equal-width bins across `extent`. Values
/// outside the extent are dropped.
pub fn bins(values: impl IntoIterator<Item = f64>, extent: Extent<f64>, count: usize) -> Vec<Bin> {
    if count == 0 || extent.is_degenerate() {
        return vec![];
    }
    let width = extent.span() / count as f64;
    let mut out: Vec<Bin> = (0..count)
        .map(|i| Bin {
            x0: extent.min + width * i as f64,
            x1: extent.min + width * (i + 1) as f64,
            count: 0,
        })
        .collect();
    for v in values.into_iter().filter(|v| v.is_finite()) {
        if v < extent.min || v > extent.max {
            continue;
        }
        let i = (((v - extent.min) / width).floor() as usize).min(count - 1);
        out[i].count += 1;
    }
    out
}

/// Trailing moving average with a window of `window` samples. The first
/// `window - 1` outputs average over the samples available so far, so the
/// output has the same length as the input.
pub fn moving_average(values: &[f64], window: usize) -> Vec<f64> {
    let window = window.max(1);
    let mut out = Vec::with_capacity(values.len());
    let mut acc = 0.0;
    let mut n = 0usize;
    for i in 0..values.len() {
        if values[i].is_finite() {
            acc += values[i];
            n += 1;
        }
        if i >= window {
            let old = values[i - window];
            if old.is_finite() {
                acc -= old;
                n -= 1;
            }
        }
        out.push(if n == 0 { f64::NAN } else { acc / n as f64 });
    }
    out
}

/// Largest-Triangle-Three-Buckets downsampling (Steinarsson 2013).
///
/// Keeps the first and last points and, in each of `threshold - 2` buckets,
/// the point forming the largest triangle with the previously kept point and
/// the next bucket's centroid. Visual shape is preserved far better than
/// naive striding. Returns the input unchanged when it already fits.
pub fn lttb(points: &[Point], threshold: usize) -> Vec<Point> {
    let n = points.len();
    if threshold >= n || threshold < 3 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(threshold);
    let bucket_size = (n - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0usize;
    out.push(points[0]);

    for i in 0..threshold - 2 {
        // Centroid of the next bucket.
        let next_start = ((i + 1) as f64 * bucket_size).floor() as usize + 1;
        let next_end = (((i + 2) as f64 * bucket_size).floor() as usize + 1).min(n);
        let next = &points[next_start..next_end];
        let (cx, cy) = next.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x, y + p.y));
        let (cx, cy) = (cx / next.len() as f64, cy / next.len() as f64);

        // Point in this bucket maximising the triangle area.
        let start = (i as f64 * bucket_size).floor() as usize + 1;
        let end = (((i + 1) as f64 * bucket_size).floor() as usize + 1).min(n);
        let pa = points[a];
        let mut best = start;
        let mut best_area = -1.0;
        for (j, p) in points.iter().enumerate().take(end).skip(start) {
            let area = ((pa.x - cx) * (p.y - pa.y) - (pa.x - p.x) * (cy - pa.y)).abs();
            if area > best_area {
                best_area = area;
                best = j;
            }
        }
        out.push(points[best]);
        a = best;
    }
    out.push(points[n - 1]);
    out
}

/// Min/max bucketing: splits `points` into `buckets` equal runs and keeps
/// the minimum and maximum of each, in x order. Unlike LTTB this never
/// drops a spike, which is what monitoring charts need.
pub fn min_max_buckets(points: &[Point], buckets: usize) -> Vec<Point> {
    let n = points.len();
    if buckets == 0 || n <= buckets * 2 {
        return points.to_vec();
    }
    let size = n as f64 / buckets as f64;
    let mut out = Vec::with_capacity(buckets * 2);
    for b in 0..buckets {
        let start = (b as f64 * size).floor() as usize;
        let end = (((b + 1) as f64 * size).floor() as usize).min(n);
        let slice = &points[start..end];
        let Some(defined) = slice.iter().find(|p| p.is_defined()) else {
            continue;
        };
        let (mut lo, mut hi) = (*defined, *defined);
        for p in slice.iter().filter(|p| p.is_defined()) {
            if p.y < lo.y {
                lo = *p;
            }
            if p.y > hi.y {
                hi = *p;
            }
        }
        if lo.x <= hi.x {
            out.push(lo);
            if lo != hi {
                out.push(hi);
            }
        } else {
            out.push(hi);
            out.push(lo);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extent_ignores_nan() {
        assert_eq!(
            extent([3.0, f64::NAN, 1.0, 2.0]),
            Some(Extent::new(1.0, 3.0))
        );
        assert_eq!(extent([f64::NAN]), None);
    }

    #[test]
    fn quantiles() {
        assert_eq!(median([1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(quantile([1.0, 2.0, 3.0, 4.0, 5.0], 0.25), Some(2.0));
        assert_eq!(mean([1.0, 2.0, f64::INFINITY, 3.0]), Some(2.0));
    }

    #[test]
    fn bins_cover_extent_and_close_last() {
        let b = bins([0.0, 0.5, 1.0, 2.0, 10.0], Extent::new(0.0, 10.0), 5);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0].count, 3);
        assert_eq!(b[1].count, 1);
        assert_eq!(b[4].count, 1, "max value lands in the last bin");
    }

    #[test]
    fn moving_average_trailing() {
        assert_eq!(
            moving_average(&[1.0, 2.0, 3.0, 4.0], 2),
            vec![1.0, 1.5, 2.5, 3.5]
        );
    }

    #[test]
    fn lttb_keeps_ends_and_spike() {
        let pts: Vec<Point> = (0..100)
            .map(|i| Point::new(i as f64, if i == 50 { 100.0 } else { 0.0 }))
            .collect();
        let out = lttb(&pts, 10);
        assert_eq!(out.len(), 10);
        assert_eq!(out[0], pts[0]);
        assert_eq!(out[9], pts[99]);
        assert!(out.iter().any(|p| p.y == 100.0), "spike survives");
        assert!(out.windows(2).all(|w| w[0].x < w[1].x), "x stays ordered");
        assert_eq!(lttb(&pts, 200).len(), 100, "no-op when it fits");
    }

    #[test]
    fn min_max_keeps_extremes_in_order() {
        let pts: Vec<Point> = (0..1000)
            .map(|i| Point::new(i as f64, ((i as f64) * 0.1).sin()))
            .collect();
        let out = min_max_buckets(&pts, 20);
        assert!(out.len() <= 40);
        assert!(out.windows(2).all(|w| w[0].x <= w[1].x));
        let max = out.iter().map(|p| p.y).fold(f64::MIN, f64::max);
        assert!(max > 0.99);
    }
}
