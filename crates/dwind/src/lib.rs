pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));


    #[macro_export]
    macro_rules! height_generator {
        ($height:tt) => {
             const_format::formatcp!("height: {};", $height)
        };
    }
}

pub mod color {
    use dwind_macros::dwgenerate_map;

    macro_rules! bg_color_generator {
       ($color:tt) => {
        const_format::formatcp!("background: {};", $color)
       };
    }

    dwgenerate_map!("bg-color-hover", "hover:bg-color-", [
        ("blue-900", "#aaaafa"),
        ("blue-800", "#9999da"),
        ("blue-700", "#8888ca"),
        ("blue-600", "#7777ba"),
        ("blue-500", "#6666aa"),
        ("blue-400", "#55559a"),
        ("blue-300", "#44448a"),
        ("blue-200", "#33337a"),
        ("blue-100", "#22226a"),
        ("blue-50", "#11115a")
    ]);
}