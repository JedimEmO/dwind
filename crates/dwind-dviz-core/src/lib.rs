//! DOM-free primitives for dwind-dviz: data model, scales, ticks, formatting,
//! path geometry, layout, statistics and palettes.
//!
//! Nothing in this crate touches the DOM, so every module is unit-tested
//! natively with `cargo test`. The renderer crate (`dwind-dviz`) consumes the
//! outputs here; a second renderer could do the same.
//!
//! Time is represented with [`jiff::Timestamp`] throughout.

pub mod data;
pub mod format;
pub mod geom;
pub mod layout;
pub mod palette;
pub mod scale;
pub mod stats;
pub mod ticks;

pub use jiff;
