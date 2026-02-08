/// A categorical scale that maps discrete categories to equal-width bands.
///
/// Used for bar chart categorical axes.
#[derive(Debug, Clone)]
pub struct BandScale {
    categories: Vec<String>,
    /// Padding between bands as a fraction of step (0.0 to 1.0).
    padding: f64,
}

impl BandScale {
    pub fn new(categories: Vec<String>, padding: f64) -> Self {
        Self {
            categories,
            padding: padding.clamp(0.0, 0.99),
        }
    }

    /// Number of categories.
    pub fn len(&self) -> usize {
        self.categories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// The step size (band + padding) in normalized [0, 1] space.
    fn step(&self) -> f64 {
        if self.categories.is_empty() {
            return 0.0;
        }
        1.0 / self.categories.len() as f64
    }

    /// The band width (excluding padding) in normalized [0, 1] space.
    pub fn bandwidth_normalized(&self) -> f64 {
        self.step() * (1.0 - self.padding)
    }

    /// The band width in pixels.
    pub fn bandwidth(&self, pixel_range: f64) -> f64 {
        self.bandwidth_normalized() * pixel_range
    }

    /// Map a category index to its center position in pixels.
    pub fn to_pixel(&self, index: usize, pixel_range: f64) -> f64 {
        let step = self.step() * pixel_range;
        step * index as f64 + step * 0.5
    }

    /// Map a category index to the start of its band in pixels.
    pub fn band_start(&self, index: usize, pixel_range: f64) -> f64 {
        let step = self.step() * pixel_range;
        let bandwidth = self.bandwidth(pixel_range);
        step * index as f64 + (step - bandwidth) * 0.5
    }

    /// Find the category index for a pixel position.
    pub fn from_pixel(&self, pixel: f64, pixel_range: f64) -> Option<usize> {
        if self.categories.is_empty() || pixel_range == 0.0 {
            return None;
        }
        let step = pixel_range / self.categories.len() as f64;
        let index = (pixel / step) as usize;
        if index < self.categories.len() {
            Some(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_band_scale() {
        let categories = vec!["A".into(), "B".into(), "C".into(), "D".into()];
        let scale = BandScale::new(categories, 0.2);

        assert_eq!(scale.len(), 4);

        // Each band takes 1/4 of the space, with 20% padding
        let bw = scale.bandwidth(400.0);
        assert!((bw - 80.0).abs() < 1e-10); // 100 * 0.8

        // Centers: at 50, 150, 250, 350
        assert!((scale.to_pixel(0, 400.0) - 50.0).abs() < 1e-10);
        assert!((scale.to_pixel(1, 400.0) - 150.0).abs() < 1e-10);
        assert!((scale.to_pixel(2, 400.0) - 250.0).abs() < 1e-10);
    }

    #[test]
    fn test_band_from_pixel() {
        let categories = vec!["A".into(), "B".into(), "C".into()];
        let scale = BandScale::new(categories, 0.1);

        assert_eq!(scale.from_pixel(10.0, 300.0), Some(0));
        assert_eq!(scale.from_pixel(110.0, 300.0), Some(1));
        assert_eq!(scale.from_pixel(250.0, 300.0), Some(2));
        assert_eq!(scale.from_pixel(310.0, 300.0), None);
    }
}
