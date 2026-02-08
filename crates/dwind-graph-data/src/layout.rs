/// Margins around the plot area inside the chart viewBox.
#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 20.0,
            right: 20.0,
            bottom: 40.0,
            left: 50.0,
        }
    }
}

/// The computed plot area rectangle within the viewBox.
#[derive(Debug, Clone, Copy)]
pub struct PlotArea {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PlotArea {
    pub fn from_viewbox(vb_width: f64, vb_height: f64, margins: &Margins) -> Self {
        Self {
            x: margins.left,
            y: margins.top,
            width: (vb_width - margins.left - margins.right).max(0.0),
            height: (vb_height - margins.top - margins.bottom).max(0.0),
        }
    }

    /// Right edge x coordinate.
    pub fn x2(&self) -> f64 {
        self.x + self.width
    }

    /// Bottom edge y coordinate.
    pub fn y2(&self) -> f64 {
        self.y + self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_area() {
        let margins = Margins {
            top: 20.0,
            right: 20.0,
            bottom: 40.0,
            left: 50.0,
        };
        let area = PlotArea::from_viewbox(800.0, 450.0, &margins);
        assert_eq!(area.x, 50.0);
        assert_eq!(area.y, 20.0);
        assert_eq!(area.width, 730.0);
        assert_eq!(area.height, 390.0);
        assert_eq!(area.x2(), 780.0);
        assert_eq!(area.y2(), 410.0);
    }
}
