use dwind_graph_data::prelude::Extent;
use futures_signals::signal::Mutable;

/// The current interaction mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractionMode {
    None,
    Pan {
        start_x: f64,
        start_y: f64,
        domain_at_start: Extent,
    },
    Brush {
        start_x: f64,
        start_y: f64,
    },
}

/// Shared chart interaction state held as Mutable signals for reactive updates.
///
/// This is created per chart instance and shared across the interaction overlay,
/// crosshair, tooltip, and data layers.
#[derive(Debug)]
pub struct ChartInteraction {
    /// Current mouse position in SVG viewBox coordinates (None when outside chart).
    pub mouse_pos: Mutable<Option<(f64, f64)>>,

    /// Index of the nearest data point: (series_index, point_index).
    pub nearest_point: Mutable<Option<(usize, usize)>>,

    /// Current interaction mode.
    pub mode: Mutable<InteractionMode>,

    /// Visible x domain after zoom/pan (None = show all data).
    pub view_domain_x: Mutable<Option<Extent>>,

    /// Visible y domain after zoom/pan (None = auto from data).
    pub view_domain_y: Mutable<Option<Extent>>,

    /// Brush selection in SVG coordinates: (start_x, start_y, end_x, end_y).
    pub brush_rect: Mutable<Option<(f64, f64, f64, f64)>>,
}

impl ChartInteraction {
    pub fn new() -> Self {
        Self {
            mouse_pos: Mutable::new(None),
            nearest_point: Mutable::new(None),
            mode: Mutable::new(InteractionMode::None),
            view_domain_x: Mutable::new(None),
            view_domain_y: Mutable::new(None),
            brush_rect: Mutable::new(None),
        }
    }

    /// Reset zoom/pan to show all data.
    pub fn reset_view(&self) {
        self.view_domain_x.set(None);
        self.view_domain_y.set(None);
    }
}

impl Default for ChartInteraction {
    fn default() -> Self {
        Self::new()
    }
}
