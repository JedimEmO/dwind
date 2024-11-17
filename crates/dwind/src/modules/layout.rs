#[macro_export]
macro_rules! aspect_generator {
    ($color:tt) => {
        const_format::formatcp!("aspect-ratio: {};", $color)
    };
}

include!(concat!(env!("OUT_DIR"), "/layout.rs"));