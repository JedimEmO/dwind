/// Result of stacking: for each series, the baseline and top values at each data index.
#[derive(Debug, Clone)]
pub struct StackedSeries {
    pub label: String,
    pub color_index: usize,
    /// Each entry is (x, y_base, y_top).
    pub data: Vec<(f64, f64, f64)>,
}

/// Compute stacked values from multiple series (for stacked bar/area charts).
///
/// All series must have the same number of data points with matching x values.
/// Returns stacked series where each y range is [cumulative_below, cumulative_below + own_y].
pub fn compute_stack(series: &[crate::types::Series]) -> Vec<StackedSeries> {
    if series.is_empty() {
        return vec![];
    }

    let n_points = series[0].data.len();
    let mut baselines = vec![0.0_f64; n_points];
    let mut result = Vec::with_capacity(series.len());

    for s in series {
        let mut stacked_data = Vec::with_capacity(n_points);
        for (i, point) in s.data.iter().enumerate() {
            let base = if i < baselines.len() {
                baselines[i]
            } else {
                0.0
            };
            let top = base + point.y;
            stacked_data.push((point.x, base, top));
            if i < baselines.len() {
                baselines[i] = top;
            }
        }
        result.push(StackedSeries {
            label: s.label.clone(),
            color_index: s.color_index,
            data: stacked_data,
        });
    }

    result
}

/// Compute the maximum stacked y value (useful for axis scaling).
pub fn stacked_y_max(stacked: &[StackedSeries]) -> f64 {
    stacked
        .iter()
        .flat_map(|s| s.data.iter().map(|(_, _, top)| *top))
        .fold(0.0_f64, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DataPoint, Series};

    #[test]
    fn test_compute_stack() {
        let series = vec![
            Series::new(
                "A",
                vec![
                    DataPoint::new(0.0, 10.0),
                    DataPoint::new(1.0, 20.0),
                    DataPoint::new(2.0, 30.0),
                ],
                0,
            ),
            Series::new(
                "B",
                vec![
                    DataPoint::new(0.0, 5.0),
                    DataPoint::new(1.0, 10.0),
                    DataPoint::new(2.0, 15.0),
                ],
                1,
            ),
        ];

        let stacked = compute_stack(&series);
        assert_eq!(stacked.len(), 2);

        // First series: base=0, top=own value
        assert_eq!(stacked[0].data[0], (0.0, 0.0, 10.0));
        assert_eq!(stacked[0].data[1], (1.0, 0.0, 20.0));

        // Second series: base=first series top
        assert_eq!(stacked[1].data[0], (0.0, 10.0, 15.0));
        assert_eq!(stacked[1].data[1], (1.0, 20.0, 30.0));
    }

    #[test]
    fn test_stacked_y_max() {
        let series = vec![
            Series::new(
                "A",
                vec![DataPoint::new(0.0, 10.0), DataPoint::new(1.0, 20.0)],
                0,
            ),
            Series::new(
                "B",
                vec![DataPoint::new(0.0, 5.0), DataPoint::new(1.0, 15.0)],
                1,
            ),
        ];
        let stacked = compute_stack(&series);
        assert_eq!(stacked_y_max(&stacked), 35.0);
    }
}
