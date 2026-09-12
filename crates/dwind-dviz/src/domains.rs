//! Deriving axis domains from series data, for presets and live charts.

use dwind_dviz_core::data::{Extent, Series, x_extent_of, y_extent_of};
use dwind_dviz_core::jiff::Timestamp;
use dwind_dviz_core::jiff::tz::TimeZone;

use crate::chart::{XDomain, YDomain};

/// What kind of x axis a preset should build from its data.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum XKind {
    /// Plain numbers.
    #[default]
    Linear,
    /// Milliseconds since the epoch (from `TimePoint::to_point`), ticked in
    /// the time zone.
    Time(TimeZone),
    /// Categories; points carry the index.
    Band(Vec<String>),
}

impl XKind {
    pub fn time_utc() -> Self {
        Self::Time(TimeZone::UTC)
    }

    pub fn band(categories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Band(categories.into_iter().map(Into::into).collect())
    }

    /// The x domain covering `series`, padded by `pad` (a fraction of the
    /// span) on both ends for non-band axes.
    pub fn domain_for(&self, series: &[Series], pad: f64) -> XDomain {
        match self {
            XKind::Linear => {
                let e = x_extent_of(series).unwrap_or(Extent::UNIT);
                XDomain::Linear(if pad > 0.0 { e.pad(pad) } else { e })
            }
            XKind::Time(tz) => {
                let e = x_extent_of(series).unwrap_or(Extent::new(0.0, 60_000.0));
                let e = if pad > 0.0 { e.pad(pad) } else { e };
                let ts = |ms: f64| {
                    Timestamp::from_millisecond(ms.round() as i64).unwrap_or(Timestamp::UNIX_EPOCH)
                };
                XDomain::Time {
                    extent: Extent::new(ts(e.min), ts(e.max)),
                    tz: tz.clone(),
                }
            }
            XKind::Band(categories) => XDomain::Band(categories.clone()),
        }
    }
}

/// A linear y domain covering `series`; `include_zero` for bars and areas,
/// `pad` (fraction of the span) for scatter so marks clear the edges.
pub fn y_domain_for(series: &[Series], include_zero: bool, pad: f64) -> YDomain {
    let e = y_extent_of(series).unwrap_or(Extent::UNIT);
    let e = if include_zero { e.include_zero() } else { e };
    YDomain::Linear(if pad > 0.0 { e.pad(pad) } else { e })
}

/// Stacked series need the y domain of their column totals, not their
/// individual extents.
pub fn y_domain_for_stacked(series: &[Series]) -> YDomain {
    let n = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
    let mut lo = 0.0f64;
    let mut hi = 0.0f64;
    for i in 0..n {
        let (mut pos, mut neg) = (0.0, 0.0);
        for s in series {
            if let Some(p) = s.points.get(i).filter(|p| p.is_defined()) {
                if p.y >= 0.0 {
                    pos += p.y;
                } else {
                    neg += p.y;
                }
            }
        }
        hi = hi.max(pos);
        lo = lo.min(neg);
    }
    YDomain::Linear(Extent::new(lo, hi))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dwind_dviz_core::data::Point;

    fn s(id: &str, ys: &[f64]) -> Series {
        Series::new(
            id,
            id,
            ys.iter()
                .enumerate()
                .map(|(i, y)| Point::new(i as f64, *y))
                .collect(),
        )
    }

    #[test]
    fn domains() {
        let all = vec![s("a", &[1.0, 5.0]), s("b", &[-2.0, 3.0])];
        assert_eq!(
            XKind::Linear.domain_for(&all, 0.0),
            XDomain::Linear(Extent::new(0.0, 1.0))
        );
        assert_eq!(
            y_domain_for(&all, false, 0.0),
            YDomain::Linear(Extent::new(-2.0, 5.0))
        );
        assert_eq!(
            y_domain_for(&[s("a", &[1.0, 5.0])], true, 0.0),
            YDomain::Linear(Extent::new(0.0, 5.0))
        );
        assert_eq!(
            y_domain_for_stacked(&all),
            YDomain::Linear(Extent::new(-2.0, 8.0))
        );
        assert!(
            matches!(XKind::band(["x"]).domain_for(&all, 0.5), XDomain::Band(c) if c == vec!["x"])
        );
        assert!(matches!(
            XKind::time_utc().domain_for(&[], 0.0),
            XDomain::Time { .. }
        ));
    }
}
