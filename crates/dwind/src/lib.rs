mod modules;

pub mod prelude {
    pub use super::base::*;
    pub use super::borders::*;
    pub use super::box_shadow::*;
    pub use super::colors::*;
    pub use super::flexbox_and_grid::*;
    pub use super::interactivity::*;
    pub use super::layout::*;
    pub use super::position::*;
    pub use super::sizing::*;
    pub use super::spacing::*;
    pub use super::typography::*;
    pub use dwind_base::*;
}

pub fn stylesheet() {
    modern_normalize_cssys::apply_modern_normalize_stylesheet();
    base::apply_base_stylesheet();
}

pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub mod box_shadow {
    include!(concat!(env!("OUT_DIR"), "/box_shadow.rs"));
}

pub use modules::borders;

pub mod interactivity {
    include!(concat!(env!("OUT_DIR"), "/interactivity.rs"));
}

pub mod layout {
    include!(concat!(env!("OUT_DIR"), "/layout.rs"));
}

pub mod position {
    include!(concat!(env!("OUT_DIR"), "/position.rs"));
}

pub mod flexbox_and_grid {
    include!(concat!(env!("OUT_DIR"), "/flexbox_and_grid.rs"));
}

pub mod typography {
    include!(concat!(env!("OUT_DIR"), "/typography.rs"));
}

pub mod transition {
    include!(concat!(env!("OUT_DIR"), "/transition.rs"));
}

pub use modules::backgrounds;
pub use modules::colors;
pub use modules::sizing;
pub use modules::spacing;
