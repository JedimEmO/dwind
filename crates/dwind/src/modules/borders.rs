use dwind_macros::dwgenerate_map;

include!(concat!(env!("OUT_DIR"), "/borders.rs"));


macro_rules! border_color_generator {
    ($color:tt) => {
        const_format::formatcp!("border-color: {};", $color)
    };
}

dwgenerate_map!("border-color", "border-color-", [
    ("manatee-50", "#f5f6f8"),
    ("manatee-100", "#ecf0f3"),
    ("manatee-200", "#dce2e9"),
    ("manatee-300", "#c7cfda"),
    ("manatee-400", "#afb9ca"),
    ("manatee-500", "#9aa3ba"),
    ("manatee-600", "#858ca8"),
    ("manatee-700", "#717791"),
    ("manatee-800", "#5d6276"),
    ("manatee-900", "#4e5261"),
    ("manatee-950", "#2e3138"),
    ("apple-50", "#f2fcf1"),
    ("apple-100", "#dff9df"),
    ("apple-200", "#c1f1c1"),
    ("apple-300", "#90e592"),
    ("apple-400", "#5cd65c"),
    ("apple-500", "#2db92d"),
    ("apple-600", "#1e8a1e"),
    ("apple-700", "#136b13"),
    ("apple-800", "#0c4f0c"),
    ("apple-900", "#073b07"),
    ("apple-950", "#042504"),
    ("bermuda-gray-50", "#f7f8f9"),
    ("bermuda-gray-100", "#ebedf0"),
    ("bermuda-gray-200", "#dfe3e8"),
    ("bermuda-gray-300", "#cfd6de"),
    ("bermuda-gray-400", "#bfc8d0"),
    ("bermuda-gray-500", "#adb8c0"),
    ("bermuda-gray-600", "#97a4b0"),
    ("bermuda-gray-700", "#7f8e9b"),
    ("bermuda-gray-800", "#667682"),
    ("bermuda-gray-900", "#4f5b6a"),
    ("bermuda-gray-950", "#2f3a44"),
    ("black", "#000000"),
    ("white", "#ffffff"),
    ("transparent", "transparent"),
    ("current", "currentColor"),
    ("inherit", "inherit"),
    ("initial", "initial"),
    ("unset", "unset"),
]);