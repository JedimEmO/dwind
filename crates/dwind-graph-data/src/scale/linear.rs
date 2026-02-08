use crate::types::Extent;

use super::traits::Scale;

/// A continuous linear scale mapping a numeric domain to [0, 1].
#[derive(Debug, Clone)]
pub struct LinearScale {
    domain: Extent,
}

impl LinearScale {
    pub fn new(domain: Extent) -> Self {
        Self { domain }
    }

    /// Create a linear scale from data, with optional padding and nice rounding.
    pub fn from_data(extent: Extent, pad: f64, nice: bool) -> Self {
        let mut domain = if nice { extent.nice() } else { extent };
        if pad > 0.0 {
            domain = domain.pad(pad);
        }
        Self { domain }
    }
}

impl Scale for LinearScale {
    fn normalize(&self, value: f64) -> f64 {
        let range = self.domain.range();
        if range == 0.0 {
            return 0.5;
        }
        (value - self.domain.min) / range
    }

    fn denormalize(&self, normalized: f64) -> f64 {
        self.domain.min + normalized * self.domain.range()
    }

    fn domain(&self) -> Extent {
        self.domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_normalize() {
        let scale = LinearScale::new(Extent::new(0.0, 100.0));
        assert!((scale.normalize(0.0) - 0.0).abs() < 1e-10);
        assert!((scale.normalize(50.0) - 0.5).abs() < 1e-10);
        assert!((scale.normalize(100.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_denormalize() {
        let scale = LinearScale::new(Extent::new(0.0, 100.0));
        assert!((scale.denormalize(0.0) - 0.0).abs() < 1e-10);
        assert!((scale.denormalize(0.5) - 50.0).abs() < 1e-10);
        assert!((scale.denormalize(1.0) - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_to_pixel() {
        let scale = LinearScale::new(Extent::new(0.0, 10.0));
        assert!((scale.to_pixel(5.0, 400.0) - 200.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_from_pixel() {
        let scale = LinearScale::new(Extent::new(0.0, 10.0));
        assert!((scale.from_pixel(200.0, 400.0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_from_data_nice() {
        let scale = LinearScale::from_data(Extent::new(0.3, 9.7), 0.0, true);
        assert_eq!(scale.domain.min, 0.0);
        assert_eq!(scale.domain.max, 10.0);
    }
}
