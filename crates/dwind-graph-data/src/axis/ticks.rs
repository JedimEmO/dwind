use crate::types::{nice_step, Extent};

/// A computed tick mark with its value and formatted label.
#[derive(Debug, Clone)]
pub struct TickSpec {
    pub value: f64,
    pub label: String,
}

/// Generate nicely-spaced tick marks for a continuous axis.
pub fn generate_ticks(extent: &Extent, target_count: usize) -> Vec<TickSpec> {
    let range = extent.range();
    if range == 0.0 || target_count == 0 {
        return vec![TickSpec {
            value: extent.min,
            label: format_tick_value(extent.min),
        }];
    }

    let step = nice_step(range, target_count);
    let start = (extent.min / step).ceil() * step;

    let mut ticks = Vec::new();
    let mut value = start;

    // Safety: limit to prevent infinite loops from floating point edge cases
    let max_ticks = target_count * 3;
    while value <= extent.max + step * 0.01 && ticks.len() < max_ticks {
        ticks.push(TickSpec {
            value,
            label: format_tick_value(value),
        });
        value += step;
    }

    ticks
}

/// Format a tick value, avoiding unnecessary decimal places.
fn format_tick_value(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let abs = value.abs();
    if abs >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.1}k", value / 1_000.0)
    } else if abs >= 1.0 {
        // Use integer formatting if the value is effectively an integer
        if (value - value.round()).abs() < 1e-9 {
            format!("{}", value.round() as i64)
        } else {
            format!("{:.1}", value)
        }
    } else {
        // Small values: show enough precision
        format!("{:.2}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ticks_basic() {
        let extent = Extent::new(0.0, 100.0);
        let ticks = generate_ticks(&extent, 5);

        assert!(!ticks.is_empty());
        assert!(ticks.len() <= 15);

        // All ticks should be within or near the extent
        for tick in &ticks {
            assert!(tick.value >= -1.0);
            assert!(tick.value <= 101.0);
        }

        // First tick should be >= min
        assert!(ticks[0].value >= 0.0);
    }

    #[test]
    fn test_generate_ticks_small_range() {
        let extent = Extent::new(0.0, 1.0);
        let ticks = generate_ticks(&extent, 5);
        assert!(!ticks.is_empty());
    }

    #[test]
    fn test_format_tick_value() {
        assert_eq!(format_tick_value(0.0), "0");
        assert_eq!(format_tick_value(50.0), "50");
        assert_eq!(format_tick_value(1500.0), "1.5k");
        assert_eq!(format_tick_value(2000000.0), "2.0M");
        assert_eq!(format_tick_value(0.25), "0.25");
    }

    #[test]
    fn test_generate_ticks_zero_range() {
        let extent = Extent::new(5.0, 5.0);
        let ticks = generate_ticks(&extent, 5);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].value, 5.0);
    }
}
