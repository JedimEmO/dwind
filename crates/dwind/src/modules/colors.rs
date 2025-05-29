use dwind_macros::dwgenerate_map;

#[macro_export]
macro_rules! bg_color_generator {
    ($color:tt) => {
        const_format::formatcp!("background: \"{}\";", $color)
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

#[macro_export]
macro_rules! shadow_color_generator {
    ($color:tt) => {
        const_format::formatcp!("border-color: {};", $color)
    };
}

#[macro_export]
macro_rules! gradient_from_generator {
    ($color:tt) => {
        const_format::formatcp!("--dw-gradient-from: {};", $color)
    };
}

#[macro_export]
macro_rules! gradient_to_generator {
    ($color:tt) => {
        const_format::formatcp!("--dw-gradient-to: {};", $color)
    };
}

#[macro_export]
macro_rules! fill_generator {
    ($color:tt) => {
        const_format::formatcp!("fill: {};", $color)
    };
}

#[macro_export]
macro_rules! stroke_generator {
    ($color:tt) => {
        const_format::formatcp!("stroke: {};", $color)
    };
}

include!(concat!(env!("OUT_DIR"), "/colors_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/colors.rs"));
