pub mod axis;
pub mod layout;
pub mod scale;
pub mod transform;
pub mod types;

pub mod prelude {
    pub use crate::axis::ticks::{generate_ticks, TickSpec};
    pub use crate::layout::{Margins, PlotArea};
    pub use crate::scale::band::BandScale;
    pub use crate::scale::linear::LinearScale;
    pub use crate::scale::traits::Scale;
    pub use crate::transform::downsample::lttb;
    pub use crate::transform::stack::{compute_stack, stacked_y_max, StackedSeries};
    pub use crate::types::{DataPoint, Extent, PieSlice, Series};
}
