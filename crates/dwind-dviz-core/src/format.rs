//! Number and time formatting for ticks, labels, tooltips and stat tiles.
//!
//! Locale-neutral on purpose: a comma thousands separator, a period decimal
//! point, and 24-hour clocks. Callers that need locale-aware output wrap
//! these with `Intl` in the renderer.

use crate::ticks::{TimeInterval, decimals_for_step};
use jiff::Timestamp;
use jiff::tz::TimeZone;

/// `1234567.891` → `"1,234,567.891"`. Keeps up to `decimals` decimals and
/// trims trailing zeros.
pub fn thousands(v: f64, decimals: usize) -> String {
    if !v.is_finite() {
        return "–".to_string();
    }
    let fixed = format!("{:.*}", decimals, v.abs());
    let (int, frac) = fixed.split_once('.').unwrap_or((&fixed, ""));
    let mut grouped = String::with_capacity(int.len() + int.len() / 3);
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let frac = frac.trim_end_matches('0');
    let mut out = String::new();
    if v < 0.0 && (int != "0" || !frac.is_empty()) {
        out.push('-');
    }
    out.push_str(&grouped);
    if !frac.is_empty() {
        out.push('.');
        out.push_str(frac);
    }
    out
}

/// `v` with exactly `decimals` decimals, no grouping.
pub fn fixed(v: f64, decimals: usize) -> String {
    if !v.is_finite() {
        return "–".to_string();
    }
    format!("{v:.decimals$}")
}

/// Formats a tick value using only the decimals its step needs:
/// `0.30000000000000004` at step `0.1` prints as `"0.3"`.
pub fn tick(v: f64, step: f64) -> String {
    thousands(v, decimals_for_step(step))
}

/// Compact SI-style magnitude: `1284` → `"1.28k"`, `4_200_000` → `"4.2M"`,
/// `0.0005` → `"500µ"`. `significant` digits total, trailing zeros trimmed.
pub fn si(v: f64, significant: usize) -> String {
    if !v.is_finite() {
        return "–".to_string();
    }
    if v == 0.0 {
        return "0".to_string();
    }
    const PREFIXES: [(f64, &str); 11] = [
        (1e18, "E"),
        (1e15, "P"),
        (1e12, "T"),
        (1e9, "G"),
        (1e6, "M"),
        (1e3, "k"),
        (1.0, ""),
        (1e-3, "m"),
        (1e-6, "µ"),
        (1e-9, "n"),
        (1e-12, "p"),
    ];
    let a = v.abs();
    let (scale, prefix) = PREFIXES
        .iter()
        .find(|(s, _)| a >= *s)
        .copied()
        .unwrap_or((1e-12, "p"));
    let scaled = v / scale;
    format!("{}{prefix}", significant_digits(scaled, significant))
}

/// Dashboard-style compact figure: `1,284` / `12.9K` / `4.2M` / `1.1B`.
/// Values below ten thousand keep their full grouped form; above, one
/// decimal with an uppercase suffix.
pub fn compact(v: f64) -> String {
    if !v.is_finite() {
        return "–".to_string();
    }
    let a = v.abs();
    let (scale, suffix) = if a >= 1e12 {
        (1e12, "T")
    } else if a >= 1e9 {
        (1e9, "B")
    } else if a >= 1e6 {
        (1e6, "M")
    } else if a >= 1e4 {
        (1e3, "K")
    } else {
        return thousands(
            v,
            if a < 10.0 {
                2
            } else if a < 100.0 {
                1
            } else {
                0
            },
        );
    };
    let scaled = v / scale;
    let decimals = if scaled.abs() >= 100.0 { 0 } else { 1 };
    format!("{}{suffix}", thousands(scaled, decimals))
}

/// `0.1234` → `"12.3%"`.
pub fn percent(fraction: f64, decimals: usize) -> String {
    if !fraction.is_finite() {
        return "–".to_string();
    }
    format!("{}%", thousands(fraction * 100.0, decimals))
}

/// A signed delta: `+12.3%` / `-4` / `±0`.
pub fn signed(v: f64, decimals: usize) -> String {
    if !v.is_finite() {
        return "–".to_string();
    }
    if v > 0.0 {
        format!("+{}", thousands(v, decimals))
    } else if v < 0.0 {
        thousands(v, decimals)
    } else {
        "±0".to_string()
    }
}

/// Human duration from milliseconds: `340ms`, `2.5s`, `1m 05s`, `1h 23m`,
/// `2d 4h`.
pub fn duration_ms(ms: f64) -> String {
    if !ms.is_finite() {
        return "–".to_string();
    }
    let neg = ms < 0.0;
    let ms = ms.abs();
    let body = if ms < 1_000.0 {
        format!("{}ms", ms.round())
    } else if ms < 60_000.0 {
        format!("{}s", significant_digits(ms / 1_000.0, 3))
    } else if ms < 3_600_000.0 {
        let total = (ms / 1_000.0).round() as u64;
        format!("{}m {:02}s", total / 60, total % 60)
    } else if ms < 86_400_000.0 {
        let total = (ms / 60_000.0).round() as u64;
        format!("{}h {:02}m", total / 60, total % 60)
    } else {
        let total = (ms / 3_600_000.0).round() as u64;
        format!("{}d {}h", total / 24, total % 24)
    };
    if neg { format!("-{body}") } else { body }
}

fn significant_digits(v: f64, significant: usize) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let magnitude = v.abs().log10().floor() as i64 + 1;
    let decimals = (significant as i64 - magnitude).max(0) as usize;
    let s = format!("{v:.decimals$}");
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

/// The strftime pattern that names a tick at `interval` resolution.
/// Coarser units show the date; finer ones the clock.
pub fn time_pattern(interval: TimeInterval) -> &'static str {
    match interval {
        TimeInterval::Millisecond(_) => "%H:%M:%S%.3f",
        TimeInterval::Second(_) => "%H:%M:%S",
        TimeInterval::Minute(_) | TimeInterval::Hour(_) => "%H:%M",
        TimeInterval::Day(_) | TimeInterval::Week(_) => "%b %d",
        TimeInterval::Month(_) => "%b %Y",
        TimeInterval::Year(_) => "%Y",
    }
}

/// A tick label at `interval` resolution in `tz`.
pub fn time(ts: Timestamp, interval: TimeInterval, tz: &TimeZone) -> String {
    ts.to_zoned(tz.clone())
        .strftime(time_pattern(interval))
        .to_string()
}

/// Labels for a whole tick run. A tick that starts a new coarser period
/// (the first tick after midnight on a clock axis, the first of a new year
/// on a month axis) shows that period instead, the way d3's multi-scale
/// format does, so the reader always knows where they are.
pub fn time_ticks(ticks: &[Timestamp], interval: TimeInterval, tz: &TimeZone) -> Vec<String> {
    ticks
        .iter()
        .map(|t| {
            let z = t.to_zoned(tz.clone());
            let pattern = match interval {
                TimeInterval::Millisecond(_)
                | TimeInterval::Second(_)
                | TimeInterval::Minute(_)
                | TimeInterval::Hour(_)
                    if z.hour() == 0
                        && z.minute() == 0
                        && z.second() == 0
                        && z.millisecond() == 0 =>
                {
                    "%b %d"
                }
                TimeInterval::Day(_) | TimeInterval::Week(_) if z.month() == 1 && z.day() == 1 => {
                    "%Y"
                }
                TimeInterval::Day(_) | TimeInterval::Week(_) if z.day() == 1 => "%b",
                TimeInterval::Month(_) if z.month() == 1 => "%Y",
                TimeInterval::Month(_) => "%b",
                other => time_pattern(other),
            };
            z.strftime(pattern).to_string()
        })
        .collect()
}

/// A full timestamp for tooltips: `2026-09-11 14:03:27`.
pub fn datetime(ts: Timestamp, tz: &TimeZone) -> String {
    ts.to_zoned(tz.clone())
        .strftime("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouping() {
        assert_eq!(thousands(1234567.891, 2), "1,234,567.89");
        assert_eq!(thousands(1000.0, 0), "1,000");
        assert_eq!(thousands(-12.5, 1), "-12.5");
        assert_eq!(thousands(0.3 + 0.6, 1), "0.9");
        assert_eq!(thousands(-0.0001, 2), "0", "no negative zero");
        assert_eq!(thousands(f64::NAN, 1), "–");
    }

    #[test]
    fn ticks_use_step_decimals() {
        assert_eq!(tick(0.1 + 0.2, 0.1), "0.3");
        assert_eq!(tick(2000.0, 500.0), "2,000");
        assert_eq!(tick(0.25, 0.05), "0.25");
    }

    #[test]
    fn si_prefixes() {
        assert_eq!(si(1284.0, 3), "1.28k");
        assert_eq!(si(4_200_000.0, 2), "4.2M");
        assert_eq!(si(0.0005, 3), "500µ");
        assert_eq!(si(999.0, 3), "999");
        assert_eq!(si(-1500.0, 2), "-1.5k");
        assert_eq!(si(0.0, 3), "0");
    }

    #[test]
    fn compact_figures() {
        assert_eq!(compact(1284.0), "1,284");
        assert_eq!(compact(12_900.0), "12.9K");
        assert_eq!(compact(4_200_000.0), "4.2M");
        assert_eq!(compact(1_100_000_000.0), "1.1B");
        assert_eq!(compact(1.23456), "1.23");
        assert_eq!(compact(42.0), "42");
        assert_eq!(compact(250_000.0), "250K");
    }

    #[test]
    fn percent_and_signed() {
        assert_eq!(percent(0.1234, 1), "12.3%");
        assert_eq!(signed(12.3, 1), "+12.3");
        assert_eq!(signed(-4.0, 0), "-4");
        assert_eq!(signed(0.0, 0), "±0");
    }

    #[test]
    fn durations() {
        assert_eq!(duration_ms(340.0), "340ms");
        assert_eq!(duration_ms(2_500.0), "2.5s");
        assert_eq!(duration_ms(65_000.0), "1m 05s");
        assert_eq!(duration_ms(4_980_000.0), "1h 23m");
        assert_eq!(duration_ms(2.0 * 86_400_000.0 + 4.0 * 3_600_000.0), "2d 4h");
    }

    fn ts(s: &str) -> Timestamp {
        s.parse().unwrap()
    }

    #[test]
    fn time_labels() {
        let utc = TimeZone::UTC;
        assert_eq!(
            time(ts("2026-09-11T14:03:27Z"), TimeInterval::Minute(15), &utc),
            "14:03"
        );
        assert_eq!(
            time(ts("2026-09-11T14:03:27Z"), TimeInterval::Day(1), &utc),
            "Sep 11"
        );
        assert_eq!(
            time(ts("2026-09-11T14:03:27Z"), TimeInterval::Month(1), &utc),
            "Sep 2026"
        );
        assert_eq!(
            datetime(ts("2026-09-11T14:03:27Z"), &utc),
            "2026-09-11 14:03:27"
        );
    }

    #[test]
    fn multi_scale_labels_mark_period_starts() {
        let utc = TimeZone::UTC;
        let run = [
            ts("2026-09-11T23:00:00Z"),
            ts("2026-09-12T00:00:00Z"),
            ts("2026-09-12T01:00:00Z"),
        ];
        assert_eq!(
            time_ticks(&run, TimeInterval::Hour(1), &utc),
            vec!["23:00", "Sep 12", "01:00"]
        );
        let months = [ts("2025-12-01T00:00:00Z"), ts("2026-01-01T00:00:00Z")];
        assert_eq!(
            time_ticks(&months, TimeInterval::Month(1), &utc),
            vec!["Dec", "2026"]
        );
    }
}
