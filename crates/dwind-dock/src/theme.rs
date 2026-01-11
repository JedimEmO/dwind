//! Theme configuration for dock components.

use serde::{Deserialize, Serialize};

/// Theme configuration for dock components.
///
/// This allows customizing the appearance of the dock system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockTheme {
    /// Tab bar height in rem.
    pub tab_bar_height: f64,
    /// Divider thickness in pixels.
    pub divider_thickness: f64,
    /// Border radius for panels (CSS value).
    pub border_radius: String,
    /// Animation duration in milliseconds.
    pub animation_duration_ms: u32,
    /// Minimum panel size in pixels.
    pub min_panel_size: f64,
    /// Drop zone indicator opacity (0.0 to 1.0).
    pub drop_zone_opacity: f64,
}

impl Default for DockTheme {
    fn default() -> Self {
        Self {
            tab_bar_height: 2.0,
            divider_thickness: 4.0,
            border_radius: "0".to_string(),
            animation_duration_ms: 150,
            min_panel_size: 100.0,
            drop_zone_opacity: 0.3,
        }
    }
}

impl DockTheme {
    /// Create a theme with rounded corners.
    pub fn rounded() -> Self {
        Self {
            border_radius: "0.25rem".to_string(),
            ..Self::default()
        }
    }

    /// Create a compact theme with smaller tab bars.
    pub fn compact() -> Self {
        Self {
            tab_bar_height: 1.5,
            divider_thickness: 2.0,
            ..Self::default()
        }
    }

    /// Builder method to set tab bar height.
    pub fn with_tab_bar_height(mut self, height: f64) -> Self {
        self.tab_bar_height = height;
        self
    }

    /// Builder method to set divider thickness.
    pub fn with_divider_thickness(mut self, thickness: f64) -> Self {
        self.divider_thickness = thickness;
        self
    }

    /// Builder method to set border radius.
    pub fn with_border_radius(mut self, radius: impl Into<String>) -> Self {
        self.border_radius = radius.into();
        self
    }

    /// Builder method to set animation duration.
    pub fn with_animation_duration(mut self, duration_ms: u32) -> Self {
        self.animation_duration_ms = duration_ms;
        self
    }
}
