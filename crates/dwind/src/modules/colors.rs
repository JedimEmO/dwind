use dwind_macros::dwgenerate_map;

macro_rules! bg_color_generator {
    ($color:tt) => {
        const_format::formatcp!("background: {};", $color)
    };
}

macro_rules! text_color_generator {
    ($color:tt) => {
        const_format::formatcp!("color: {};", $color)
    };
}

dwgenerate_map!(
    "bg",
    "bg-color-",
    [
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
    ]
);

dwgenerate_map!(
    "text",
    "text-color-",
    [
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
    ]
);

// Apple

dwgenerate_map!(
    "bg",
    "bg-color-",
    [
        ("apple-50", "#f2fcf1"),
        ("apple-100", "#dff9df"),
        ("apple-200", "#c1f1c1"),
        ("apple-300", "#90e592"),
        ("apple-400", "#58d05b"),
        ("apple-500", "#32b835"),
        ("apple-600", "#239625"),
        ("apple-700", "#1f7622"),
        ("apple-800", "#1d5e20"),
        ("apple-900", "#1a4d1c"),
        ("apple-950", "#092a0c"),
    ]
);

dwgenerate_map!(
    "text",
    "text-color-",
    [
        ("apple-50", "#f2fcf1"),
        ("apple-100", "#dff9df"),
        ("apple-200", "#c1f1c1"),
        ("apple-300", "#90e592"),
        ("apple-400", "#58d05b"),
        ("apple-500", "#32b835"),
        ("apple-600", "#239625"),
        ("apple-700", "#1f7622"),
        ("apple-800", "#1d5e20"),
        ("apple-900", "#1a4d1c"),
        ("apple-950", "#092a0c"),
    ]
);

// Purple
dwgenerate_map!(
    "bg",
    "bg-color-",
    [
        ("purple-50", "#fbf3ff"),
        ("purple-100", "#f6e2ff"),
        ("purple-200", "#eecbff"),
        ("purple-300", "#e1a2ff"),
        ("purple-400", "#cf67ff"),
        ("purple-500", "#be2eff"),
        ("purple-600", "#ae07ff"),
        ("purple-700", "#9900f7"),
        ("purple-800", "#8201c8"),
        ("purple-900", "#5f028f"),
        ("purple-950", "#4a007a"),
    ]
);

dwgenerate_map!(
    "text",
    "text-color-",
    [
        ("purple-50", "#fbf3ff"),
        ("purple-100", "#f6e2ff"),
        ("purple-200", "#eecbff"),
        ("purple-300", "#e1a2ff"),
        ("purple-400", "#cf67ff"),
        ("purple-500", "#be2eff"),
        ("purple-600", "#ae07ff"),
        ("purple-700", "#9900f7"),
        ("purple-800", "#8201c8"),
        ("purple-900", "#5f028f"),
        ("purple-950", "#4a007a"),
    ]
);

// Bermuda Gray
dwgenerate_map!(
    "bg",
    "bg-color-",
    [
        ("bermuda-gray-50", "#f4f7f9"),
        ("bermuda-gray-100", "#ebf1f4"),
        ("bermuda-gray-200", "#dbe5ea"),
        ("bermuda-gray-300", "#c5d4dc"),
        ("bermuda-gray-400", "#adbfcc"),
        ("bermuda-gray-500", "#97abbd"),
        ("bermuda-gray-600", "#7b8ea7"),
        ("bermuda-gray-700", "#6e7e94"),
        ("bermuda-gray-800", "#5a6779"),
        ("bermuda-gray-900", "#4c5663"),
        ("bermuda-gray-950", "#2d3339")
    ]
);

dwgenerate_map!(
    "text",
    "text-color-",
    [
        ("bermuda-gray-50", "#f4f7f9"),
        ("bermuda-gray-100", "#ebf1f4"),
        ("bermuda-gray-200", "#dbe5ea"),
        ("bermuda-gray-300", "#c5d4dc"),
        ("bermuda-gray-400", "#adbfcc"),
        ("bermuda-gray-500", "#97abbd"),
        ("bermuda-gray-600", "#7b8ea7"),
        ("bermuda-gray-700", "#6e7e94"),
        ("bermuda-gray-800", "#5a6779"),
        ("bermuda-gray-900", "#4c5663"),
        ("bermuda-gray-950", "#2d3339")
    ]
);

// Candlelight
dwgenerate_map!(
    "bg",
    "bg-color-",
    [
        ("candlelight-50", "#fefce8"),
        ("candlelight-100", "#fffac2"),
        ("candlelight-200", "#fff089"),
        ("candlelight-300", "#ffe042"),
        ("candlelight-400", "#fdcd12"),
        ("candlelight-500", "#ecb306"),
        ("candlelight-600", "#cc8a02"),
        ("candlelight-700", "#a36105"),
        ("candlelight-800", "#864c0d"),
        ("candlelight-900", "#723f11"),
        ("candlelight-950", "#432005"),
    ]
);

dwgenerate_map!(
    "bg-hover",
    "hover:bg-color-",
    [
        ("candlelight-50", "#fefce8"),
        ("candlelight-100", "#fffac2"),
        ("candlelight-200", "#fff089"),
        ("candlelight-300", "#ffe042"),
        ("candlelight-400", "#fdcd12"),
        ("candlelight-500", "#ecb306"),
        ("candlelight-600", "#cc8a02"),
        ("candlelight-700", "#a36105"),
        ("candlelight-800", "#864c0d"),
        ("candlelight-900", "#723f11"),
        ("candlelight-950", "#432005"),
    ]
);