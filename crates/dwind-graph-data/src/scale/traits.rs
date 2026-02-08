use crate::types::Extent;

/// Maps values from a data domain to a normalized [0, 1] range and back.
pub trait Scale {
    /// Map a domain value to the [0.0, 1.0] normalized range.
    fn normalize(&self, value: f64) -> f64;

    /// Map a normalized [0.0, 1.0] value back to the domain.
    fn denormalize(&self, normalized: f64) -> f64;

    /// Map a domain value directly to a pixel coordinate.
    fn to_pixel(&self, value: f64, pixel_range: f64) -> f64 {
        self.normalize(value) * pixel_range
    }

    /// Map a pixel coordinate back to a domain value.
    fn from_pixel(&self, pixel: f64, pixel_range: f64) -> f64 {
        self.denormalize(pixel / pixel_range)
    }

    /// The domain extent of this scale.
    fn domain(&self) -> Extent;
}
