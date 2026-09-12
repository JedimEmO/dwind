//! Reactive, accessible SVG charts for dwind / dwui / dominator.
//!
//! A chart is a [`chart::chart`] container plus a list of layers. The
//! container owns the size (through a `ResizeObserver`), the domains, and
//! the resulting [`chart::Frame`] of scales; every layer reads that frame as
//! a signal and patches its own attributes. Lines are one `<path>` whose
//! `d` is a string signal, so a live series costs one attribute update per
//! change, not a rebuild.
//!
//! See PLAN.md for the architecture and the phase this crate is in.

pub mod chart;
pub mod domains;
pub(crate) mod keyed;
pub mod layers;
pub mod legend;
pub mod live;
pub mod motion;
pub mod presets;
pub mod theme;
pub mod visibility;

pub use dwind_dviz_core as core;
pub use dwind_dviz_data as data;

pub mod prelude {
    pub use crate::chart::*;
    pub use crate::domains::{XKind, y_domain_for, y_domain_for_stacked};
    pub use crate::layers::annotations::{band_y, reference_x, reference_y};
    pub use crate::layers::arc::{ArcOptions, arcs};
    pub use crate::layers::area::{AreaOptions, area, area_with};
    pub use crate::layers::axis::{axis_x, axis_y};
    pub use crate::layers::bars::{BarMode, BarOptions, bars, bars_with};
    pub use crate::layers::cells::{Cell, CellOptions, CellScale, cells};
    pub use crate::layers::grid::grid;
    pub use crate::layers::interaction::{CrosshairOptions, crosshair, crosshair_with};
    pub use crate::layers::labels::{bar_value_labels, line_end_labels};
    pub use crate::layers::line::{LineOptions, Wash, line, line_with};
    pub use crate::layers::points::{PointOptions, points, points_with};
    pub use crate::layers::width_tap::width_tap;
    pub use crate::legend::{legend, legend_with};
    pub use crate::live::{commit_on_frame, live_indicator};
    pub use crate::presets::*;
    pub use crate::theme::{apply_style_sheet, set_mode};
    pub use crate::visibility::SeriesVisibility;
    pub use dwind_dviz_core::data::{Extent, Point, Series, TimePoint};
    pub use dwind_dviz_core::geom::Curve;
    pub use dwind_dviz_core::palette::Mode;
    pub use dwind_dviz_data::{Retention, Strategy, WindowedSource, downsample, drive};
}
