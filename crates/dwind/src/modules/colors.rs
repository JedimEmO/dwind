use dwind_macros::dwgenerate_map;

#[macro_export]
macro_rules! bg_color_generator {
    ($color:tt) => {
        const_format::formatcp!("background: {};", $color)
    };
}

#[macro_export]
macro_rules! text_color_generator {
    ($color:tt) => {
        const_format::formatcp!("color: {};", $color)
    };
}

#[macro_export]
macro_rules! border_color_generator {
    ($color:tt) => {
        const_format::formatcp!("border-color: {};", $color)
    };
}

include!(concat!(env!("OUT_DIR"), "/colors_generated.rs"));
