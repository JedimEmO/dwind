//! Capping points per series to what the plot can show.

use dwind_dviz_core::data::Series;
use dwind_dviz_core::stats::{lttb, min_max_buckets};
use futures_signals::map_ref;
use futures_signals::signal::Signal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strategy {
    /// Per bucket keep the minimum and the maximum: a spike is never lost.
    /// The right default for monitoring.
    #[default]
    MinMax,
    /// Largest-Triangle-Three-Buckets: preserves the visual shape with
    /// fewer points, but can drop a one-sample spike.
    Lttb,
}

/// Reduces each series to about `2 × width` points when it has more,
/// where `width` is the plot width in px (one bucket per pixel).
pub fn reduce(series: &[Series], width: f64, strategy: Strategy) -> Vec<Series> {
    let buckets = (width.max(1.0)).floor() as usize;
    series
        .iter()
        .map(|s| {
            if s.points.len() <= buckets * 2 {
                return s.clone();
            }
            let points = match strategy {
                Strategy::MinMax => min_max_buckets(&s.points, buckets),
                Strategy::Lttb => lttb(&s.points, buckets * 2),
            };
            Series::new(s.id.clone(), s.label.clone(), points)
        })
        .collect()
}

/// A signal of the reduced series, following both the data and the width.
pub fn downsample<S, W>(series: S, width: W, strategy: Strategy) -> impl Signal<Item = Vec<Series>>
where
    S: Signal<Item = Vec<Series>>,
    W: Signal<Item = f64>,
{
    map_ref! {
        let all = series,
        let w = width => reduce(all, *w, strategy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dwind_dviz_core::data::Point;

    #[test]
    fn reduces_only_when_needed_and_keeps_spikes() {
        let big: Vec<Point> = (0..10_000)
            .map(|i| Point::new(i as f64, if i == 5_000 { 100.0 } else { 0.0 }))
            .collect();
        let all = vec![
            Series::new("a", "a", big),
            Series::new("b", "b", vec![Point::new(0.0, 1.0)]),
        ];
        let out = reduce(&all, 400.0, Strategy::MinMax);
        assert!(out[0].points.len() <= 800);
        assert!(out[0].points.iter().any(|p| p.y == 100.0));
        assert_eq!(out[1].points.len(), 1, "small series untouched");
        let l = reduce(&all, 400.0, Strategy::Lttb);
        assert_eq!(l[0].points.len(), 800);
    }
}
