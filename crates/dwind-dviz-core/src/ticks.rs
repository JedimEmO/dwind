//! Tick generation for linear, logarithmic and time axes.
//!
//! Linear ticks follow the d3 convention: steps are 1, 2 or 5 times a power
//! of ten, chosen so that roughly `count` ticks fit. Time ticks pick a
//! calendar-aware interval and snap to its boundaries in a time zone.

use crate::data::Extent;
use jiff::civil::Weekday;
use jiff::tz::TimeZone;
use jiff::{Span, Timestamp, ToSpan, Zoned};

const E10: f64 = 7.071_067_811_865_476; // sqrt(50)
const E5: f64 = 3.162_277_660_168_379_5; // sqrt(10)
const E2: f64 = std::f64::consts::SQRT_2;

/// The tick step for `count` ticks across `[start, stop]`, always a
/// 1/2/5 × 10^n value. Negative when `stop < start`.
pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    let count = count.max(1) as f64;
    let span = (stop - start).abs();
    if span == 0.0 || !span.is_finite() {
        return 0.0;
    }
    let step = span / count;
    let power = step.log10().floor();
    let error = step / 10f64.powf(power);
    let factor = if error >= E10 {
        10.0
    } else if error >= E5 {
        5.0
    } else if error >= E2 {
        2.0
    } else {
        1.0
    };
    let step = factor * 10f64.powf(power);
    if stop < start { -step } else { step }
}

/// Evenly spaced round ticks inside `[start, stop]`, inclusive of the ends
/// when they land on a step.
pub fn linear_ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    if start == stop {
        return vec![start];
    }
    let step = tick_step(start, stop, count);
    if step == 0.0 || !step.is_finite() {
        return vec![];
    }
    let (lo, hi, step) = if step > 0.0 {
        (start, stop, step)
    } else {
        (stop, start, -step)
    };
    let i0 = (lo / step).ceil() as i64;
    let i1 = (hi / step).floor() as i64;
    let mut ticks: Vec<f64> = (i0..=i1).map(|i| snap(i as f64 * step, step)).collect();
    if start > stop {
        ticks.reverse();
    }
    ticks
}

/// Kills floating-point dust such as `0.30000000000000004`.
fn snap(v: f64, step: f64) -> f64 {
    let decimals = decimals_for_step(step) as i32;
    let scale = 10f64.powi(decimals);
    (v * scale).round() / scale
}

/// Number of decimals needed to print values stepped by `step`.
pub fn decimals_for_step(step: f64) -> usize {
    if step == 0.0 || !step.is_finite() {
        return 0;
    }
    (-(step.abs().log10().floor())).max(0.0) as usize
}

/// Extends `extent` outward to the nearest tick boundaries, so the axis
/// starts and ends on round numbers.
pub fn nice_extent(extent: Extent<f64>, count: usize) -> Extent<f64> {
    if extent.is_degenerate() {
        return extent;
    }
    let mut lo = extent.min;
    let mut hi = extent.max;
    // Two passes, as d3 does: the step can change once the ends move.
    for _ in 0..2 {
        let step = tick_step(lo, hi, count);
        if step <= 0.0 {
            break;
        }
        let new_lo = snap((lo / step).floor() * step, step);
        let new_hi = snap((hi / step).ceil() * step, step);
        if new_lo == lo && new_hi == hi {
            break;
        }
        lo = new_lo;
        hi = new_hi;
    }
    Extent { min: lo, max: hi }
}

/// Ticks for a logarithmic axis: every power of `base` inside the extent,
/// with the 2..base-1 multiples added when at most `count / 2` decades are
/// shown. The extent must be positive.
pub fn log_ticks(extent: Extent<f64>, base: f64, count: usize) -> Vec<f64> {
    if extent.min <= 0.0 || extent.max <= 0.0 || base <= 1.0 {
        return vec![];
    }
    let lo = extent.min.log(base).floor() as i32;
    let hi = extent.max.log(base).ceil() as i32;
    let decades = (hi - lo) as usize;
    let minor = decades <= count / 2 && base.fract() == 0.0 && base <= 10.0;
    let mut out = Vec::new();
    for p in lo..=hi {
        let decade = base.powi(p);
        let mults: Vec<f64> = if minor {
            (1..base as u32).map(f64::from).collect()
        } else {
            vec![1.0]
        };
        for m in mults {
            let v = decade * m;
            if v >= extent.min * (1.0 - 1e-9) && v <= extent.max * (1.0 + 1e-9) {
                out.push(v);
            }
        }
    }
    out
}

/// A calendar interval between time ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeInterval {
    Millisecond(u32),
    Second(u32),
    Minute(u32),
    Hour(u32),
    Day(u32),
    Week(u32),
    Month(u32),
    Year(u32),
}

impl TimeInterval {
    /// Approximate length in milliseconds, for choosing an interval.
    pub fn approx_ms(self) -> f64 {
        const S: f64 = 1_000.0;
        match self {
            Self::Millisecond(n) => n as f64,
            Self::Second(n) => n as f64 * S,
            Self::Minute(n) => n as f64 * 60.0 * S,
            Self::Hour(n) => n as f64 * 3_600.0 * S,
            Self::Day(n) => n as f64 * 86_400.0 * S,
            Self::Week(n) => n as f64 * 7.0 * 86_400.0 * S,
            Self::Month(n) => n as f64 * 30.436_875 * 86_400.0 * S,
            Self::Year(n) => n as f64 * 365.2425 * 86_400.0 * S,
        }
    }

    fn span(self) -> Span {
        match self {
            Self::Millisecond(n) => (n as i64).milliseconds(),
            Self::Second(n) => (n as i64).seconds(),
            Self::Minute(n) => (n as i64).minutes(),
            Self::Hour(n) => (n as i64).hours(),
            Self::Day(n) => (n as i64).days(),
            Self::Week(n) => (n as i64).weeks(),
            Self::Month(n) => (n as i64).months(),
            Self::Year(n) => (n as i64).years(),
        }
    }

    /// The candidate intervals an axis chooses from.
    pub const CANDIDATES: [TimeInterval; 27] = [
        Self::Millisecond(1),
        Self::Millisecond(5),
        Self::Millisecond(10),
        Self::Millisecond(50),
        Self::Millisecond(100),
        Self::Millisecond(500),
        Self::Second(1),
        Self::Second(5),
        Self::Second(15),
        Self::Second(30),
        Self::Minute(1),
        Self::Minute(5),
        Self::Minute(15),
        Self::Minute(30),
        Self::Hour(1),
        Self::Hour(3),
        Self::Hour(6),
        Self::Hour(12),
        Self::Day(1),
        Self::Day(2),
        Self::Week(1),
        Self::Month(1),
        Self::Month(3),
        Self::Month(6),
        Self::Year(1),
        Self::Year(5),
        Self::Year(10),
    ];

    /// Chooses the candidate whose length is closest (in log space) to
    /// `span_ms / count`. Beyond ten years, multiplies years by a round
    /// factor.
    pub fn choose(span_ms: f64, count: usize) -> TimeInterval {
        let target = span_ms / count.max(1) as f64;
        if target <= 0.0 || !target.is_finite() {
            return Self::Second(1);
        }
        if target > Self::Year(10).approx_ms() {
            let years = target / Self::Year(1).approx_ms();
            let step = tick_step(0.0, years, 1).max(10.0);
            return Self::Year(step as u32);
        }
        *Self::CANDIDATES
            .iter()
            .min_by(|a, b| {
                let da = (a.approx_ms().ln() - target.ln()).abs();
                let db = (b.approx_ms().ln() - target.ln()).abs();
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    /// Rounds `z` down to the start of this interval's grid.
    pub fn floor(self, z: &Zoned) -> Zoned {
        let day_start = || z.start_of_day().unwrap_or_else(|_| z.clone());
        match self {
            Self::Millisecond(n) => {
                let ms = z.timestamp().as_millisecond();
                let n = n as i64;
                Timestamp::from_millisecond(ms.div_euclid(n) * n)
                    .unwrap()
                    .to_zoned(z.time_zone().clone())
            }
            Self::Second(n) => {
                let s = z.timestamp().as_second();
                let n = n as i64;
                Timestamp::from_second(s.div_euclid(n) * n)
                    .unwrap()
                    .to_zoned(z.time_zone().clone())
            }
            Self::Minute(n) => {
                let minutes = (z.hour() as i64) * 60 + z.minute() as i64;
                let floored = minutes - minutes % n as i64;
                day_start().checked_add(floored.minutes()).unwrap()
            }
            Self::Hour(n) => {
                let h = z.hour() as i64;
                day_start().checked_add((h - h % n as i64).hours()).unwrap()
            }
            Self::Day(n) => {
                let d = (z.day() - 1) as i64;
                day_start().checked_sub((d % n as i64).days()).unwrap()
            }
            Self::Week(_) => {
                let back = z.weekday().to_monday_zero_offset() as i64;
                day_start().checked_sub(back.days()).unwrap()
            }
            Self::Month(n) => {
                let m = (z.month() - 1) as i64;
                let first = z.first_of_month().unwrap().start_of_day().unwrap();
                first.checked_sub((m % n as i64).months()).unwrap()
            }
            Self::Year(n) => {
                let y = z.year() as i64;
                let first = z.first_of_year().unwrap().start_of_day().unwrap();
                first.checked_sub(y.rem_euclid(n as i64).years()).unwrap()
            }
        }
    }

    pub fn next(self, z: &Zoned) -> Zoned {
        z.checked_add(self.span()).unwrap_or_else(|_| z.clone())
    }
}

/// Time ticks and the interval that produced them (the label format
/// depends on it).
#[derive(Debug, Clone, PartialEq)]
pub struct TimeTicks {
    pub interval: TimeInterval,
    pub ticks: Vec<Timestamp>,
}

/// Roughly `count` calendar-aligned ticks inside `extent`, in `tz`.
pub fn time_ticks(extent: Extent<Timestamp>, count: usize, tz: &TimeZone) -> TimeTicks {
    let interval = TimeInterval::choose(extent.span_ms(), count);
    let ticks = time_ticks_with(extent, interval, tz);
    TimeTicks { interval, ticks }
}

/// Ticks at every `interval` boundary inside `extent`.
pub fn time_ticks_with(
    extent: Extent<Timestamp>,
    interval: TimeInterval,
    tz: &TimeZone,
) -> Vec<Timestamp> {
    let start = extent.min.to_zoned(tz.clone());
    let mut cursor = interval.floor(&start);
    if cursor.timestamp() < extent.min {
        cursor = interval.next(&cursor);
    }
    let mut out = Vec::new();
    // Hard cap so a bad interval can never spin forever.
    while cursor.timestamp() <= extent.max && out.len() < 10_000 {
        out.push(cursor.timestamp());
        let next = interval.next(&cursor);
        if next.timestamp() <= cursor.timestamp() {
            break;
        }
        cursor = next;
    }
    out
}

/// Weekday of a timestamp in `tz`.
pub fn weekday(ts: Timestamp, tz: &TimeZone) -> Weekday {
    ts.to_zoned(tz.clone()).weekday()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_are_1_2_5() {
        assert_eq!(tick_step(0.0, 100.0, 10), 10.0);
        assert_eq!(tick_step(0.0, 100.0, 5), 20.0);
        assert_eq!(tick_step(0.0, 1.0, 4), 0.2);
        assert_eq!(tick_step(0.0, 7.0, 10), 0.5);
        assert_eq!(tick_step(100.0, 0.0, 10), -10.0);
    }

    #[test]
    fn linear_ticks_are_clean() {
        assert_eq!(
            linear_ticks(0.0, 1.0, 5),
            vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]
        );
        assert_eq!(linear_ticks(-1.0, 1.0, 4), vec![-1.0, -0.5, 0.0, 0.5, 1.0]);
        assert_eq!(
            linear_ticks(3.0, 97.0, 10),
            (10..=90).step_by(10).map(f64::from).collect::<Vec<_>>()
        );
        assert_eq!(linear_ticks(1.0, 0.0, 2), vec![1.0, 0.5, 0.0]);
        assert_eq!(linear_ticks(5.0, 5.0, 5), vec![5.0]);
    }

    #[test]
    fn nice_extent_rounds_out() {
        assert_eq!(
            nice_extent(Extent::new(3.0, 97.0), 10),
            Extent::new(0.0, 100.0)
        );
        assert_eq!(
            nice_extent(Extent::new(0.13, 0.87), 5),
            Extent::new(0.0, 1.0)
        );
        assert_eq!(
            nice_extent(Extent::new(-12.0, 88.0), 5),
            Extent::new(-20.0, 100.0)
        );
    }

    #[test]
    fn decimals() {
        assert_eq!(decimals_for_step(10.0), 0);
        assert_eq!(decimals_for_step(0.5), 1);
        assert_eq!(decimals_for_step(0.05), 2);
    }

    #[test]
    fn log_ticks_decades_and_minors() {
        assert_eq!(
            log_ticks(Extent::new(1.0, 1000.0), 10.0, 3),
            vec![1.0, 10.0, 100.0, 1000.0]
        );
        let fine = log_ticks(Extent::new(1.0, 100.0), 10.0, 10);
        assert!(fine.contains(&2.0) && fine.contains(&50.0) && fine.contains(&100.0));
        assert!(log_ticks(Extent::new(0.0, 10.0), 10.0, 5).is_empty());
    }

    fn ts(s: &str) -> Timestamp {
        s.parse().unwrap()
    }

    #[test]
    fn chooses_sensible_intervals() {
        let hour = 3_600_000.0;
        assert_eq!(TimeInterval::choose(hour, 6), TimeInterval::Minute(15));
        assert_eq!(TimeInterval::choose(24.0 * hour, 8), TimeInterval::Hour(3));
        assert_eq!(
            TimeInterval::choose(365.0 * 24.0 * hour, 12),
            TimeInterval::Month(1)
        );
        assert_eq!(TimeInterval::choose(60_000.0, 6), TimeInterval::Second(15));
        assert_eq!(
            TimeInterval::choose(1_000.0, 10),
            TimeInterval::Millisecond(100)
        );
        assert!(matches!(TimeInterval::choose(1e13, 5), TimeInterval::Year(n) if n >= 50));
    }

    #[test]
    fn minute_ticks_snap_to_boundaries() {
        let e = Extent::new(ts("2026-03-01T10:07:00Z"), ts("2026-03-01T11:02:00Z"));
        let t = time_ticks(e, 6, &TimeZone::UTC);
        assert_eq!(t.interval, TimeInterval::Minute(15));
        assert_eq!(
            t.ticks,
            vec![
                ts("2026-03-01T10:15:00Z"),
                ts("2026-03-01T10:30:00Z"),
                ts("2026-03-01T10:45:00Z"),
                ts("2026-03-01T11:00:00Z")
            ]
        );
    }

    #[test]
    fn second_ticks_snap() {
        let e = Extent::new(ts("2026-03-01T10:00:07Z"), ts("2026-03-01T10:00:31Z"));
        let t = time_ticks_with(e, TimeInterval::Second(5), &TimeZone::UTC);
        assert_eq!(t[0], ts("2026-03-01T10:00:10Z"));
        assert_eq!(t.len(), 5);
    }

    #[test]
    fn day_ticks_respect_time_zone_and_dst() {
        let tz = TimeZone::get("Europe/Oslo").unwrap();
        // Spans the March 2026 DST change (29 March).
        let e = Extent::new(ts("2026-03-27T12:00:00Z"), ts("2026-03-31T12:00:00Z"));
        let ticks = time_ticks_with(e, TimeInterval::Day(1), &tz);
        assert_eq!(ticks.len(), 4);
        for t in &ticks {
            let z = t.to_zoned(tz.clone());
            assert_eq!((z.hour(), z.minute()), (0, 0), "local midnight: {z}");
        }
    }

    #[test]
    fn month_and_year_ticks() {
        let e = Extent::new(ts("2025-11-15T00:00:00Z"), ts("2026-03-15T00:00:00Z"));
        let t = time_ticks_with(e, TimeInterval::Month(1), &TimeZone::UTC);
        assert_eq!(t.len(), 4);
        assert_eq!(t[0], ts("2025-12-01T00:00:00Z"));
        let y = time_ticks_with(
            Extent::new(ts("2003-06-01T00:00:00Z"), ts("2026-01-01T00:00:00Z")),
            TimeInterval::Year(5),
            &TimeZone::UTC,
        );
        assert_eq!(y[0], ts("2005-01-01T00:00:00Z"));
        assert_eq!(*y.last().unwrap(), ts("2025-01-01T00:00:00Z"));
    }

    #[test]
    fn week_ticks_start_monday() {
        let e = Extent::new(ts("2026-09-01T00:00:00Z"), ts("2026-09-30T00:00:00Z"));
        let t = time_ticks_with(e, TimeInterval::Week(1), &TimeZone::UTC);
        assert!(
            t.iter()
                .all(|x| weekday(*x, &TimeZone::UTC) == Weekday::Monday)
        );
        assert_eq!(t[0], ts("2026-09-07T00:00:00Z"));
    }

    #[test]
    fn empty_extent_yields_no_spin() {
        let e = Extent::new(ts("2026-01-01T00:00:00Z"), ts("2026-01-01T00:00:00Z"));
        let t = time_ticks(e, 5, &TimeZone::UTC);
        assert!(t.ticks.len() <= 1);
    }
}
