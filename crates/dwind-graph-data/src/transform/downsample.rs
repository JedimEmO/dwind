use crate::types::DataPoint;

/// Largest Triangle Three Buckets (LTTB) downsampling algorithm.
///
/// Reduces a data series to `threshold` points while preserving the visual shape
/// of the line. The first and last points are always included.
///
/// If the data length is <= threshold, returns a clone of the input.
pub fn lttb(data: &[DataPoint], threshold: usize) -> Vec<DataPoint> {
    let len = data.len();

    if threshold >= len || threshold < 3 {
        return data.to_vec();
    }

    let mut result = Vec::with_capacity(threshold);

    // Always include the first point
    result.push(data[0]);

    let bucket_size = (len - 2) as f64 / (threshold - 2) as f64;

    let mut prev_selected_index: usize = 0;

    for i in 0..(threshold - 2) {
        // Calculate the average point of the next bucket (used as target)
        let avg_start = ((i + 1) as f64 * bucket_size) as usize + 1;
        let avg_end = (((i + 2) as f64 * bucket_size) as usize + 1).min(len);

        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        let avg_count = (avg_end - avg_start) as f64;

        for j in avg_start..avg_end {
            avg_x += data[j].x;
            avg_y += data[j].y;
        }
        avg_x /= avg_count;
        avg_y /= avg_count;

        // Calculate the current bucket range
        let bucket_start = (i as f64 * bucket_size) as usize + 1;
        let bucket_end = ((i + 1) as f64 * bucket_size) as usize + 1;

        // Find the point in the current bucket with the largest triangle area
        let prev = data[prev_selected_index];
        let mut max_area = -1.0;
        let mut max_index = bucket_start;

        for j in bucket_start..bucket_end {
            // Triangle area (using cross product formula, without the 0.5 factor)
            let area = ((prev.x - avg_x) * (data[j].y - prev.y)
                - (prev.x - data[j].x) * (avg_y - prev.y))
                .abs();

            if area > max_area {
                max_area = area;
                max_index = j;
            }
        }

        result.push(data[max_index]);
        prev_selected_index = max_index;
    }

    // Always include the last point
    result.push(data[len - 1]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lttb_passthrough_small() {
        let data: Vec<DataPoint> = (0..5).map(|i| DataPoint::new(i as f64, i as f64)).collect();
        let result = lttb(&data, 10);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_lttb_reduces() {
        let data: Vec<DataPoint> = (0..100)
            .map(|i| DataPoint::new(i as f64, (i as f64 * 0.1).sin()))
            .collect();
        let result = lttb(&data, 20);
        assert_eq!(result.len(), 20);

        // First and last points preserved
        assert_eq!(result[0], data[0]);
        assert_eq!(result[19], data[99]);
    }

    #[test]
    fn test_lttb_preserves_shape() {
        // Create data with a clear peak
        let mut data = Vec::new();
        for i in 0..50 {
            data.push(DataPoint::new(i as f64, i as f64));
        }
        for i in 50..100 {
            data.push(DataPoint::new(i as f64, (100 - i) as f64));
        }

        let result = lttb(&data, 10);
        // The peak area (around index 50) should be represented
        let has_peak_area = result.iter().any(|p| p.y > 40.0);
        assert!(has_peak_area);
    }
}
