use dwind_macros::dwgenerate_map;

#[macro_export]
macro_rules! background_scratched_generator {
    ($color1:tt, $color2:tt) => {
        const_format::formatcp!(
            "background-image: repeating-linear-gradient(45deg, {}, {} 4px, {} 4px, {} 8px);",
            $color1,
            $color1,
            $color2,
            $color2
        )
    };
}
