pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub mod interactivity {
    include!(concat!(env!("OUT_DIR"), "/interactivity.rs"));
}


pub mod flexbox_and_grid {
    include!(concat!(env!("OUT_DIR"), "/flexbox_and_grid.rs"));
}

pub mod spacing {
    use dwind_macros::dwgenerate_map;

    include!(concat!(env!("OUT_DIR"), "/spacing.rs"));

    #[macro_export]
    macro_rules! margin_x_generator {
        ($margin:tt) => {
            const_format::formatcp!("margin-left: {};margin-right: {};", $margin, $margin)
        };
    }

    #[macro_export]
    macro_rules! margin_generator {
        ($dir:tt, $margin:tt) => {
            const_format::formatcp!("margin-{}: {};", $dir, $margin)
        };
    }

    dwgenerate_map!("m", "margin-", [
        ("l-0", "left,0"),
        ("l-1", "left,4px"),
        ("l-2", "left,8px"),
        ("l-3", "left,12px"),
        ("l-4", "left,16px"),
        ("l-5", "left,20px"),
        ("l-6", "left,24px"),
        ("l-7", "left,28px"),
        ("l-8", "left,32px"),
        ("l-9", "left,36px"),
        ("l-10", "left,40px"),
        ("l-11", "left,44px"),
        ("l-12", "left,48px"),
        ("l-13", "left,52px"),
        ("l-14", "left,56px"),
        ("l-15", "left,60px"),
        ("l-16", "left,64px"),
        ("l-17", "left,68px"),
        ("l-18", "left,72px"),
        ("l-19", "left,76px"),
        ("l-20", "left,80px"),
        ("r-0", "right,0"),
        ("r-1", "right,4px"),
        ("r-2", "right,8px"),
        ("r-3", "right,12px"),
        ("r-4", "right,16px"),
        ("r-5", "right,20px"),
        ("r-6", "right,24px"),
        ("r-7", "right,28px"),
        ("r-8", "right,32px"),
        ("r-9", "right,36px"),
        ("r-10", "right,40px"),
        ("r-11", "right,44px"),
        ("r-12", "right,48px"),
        ("r-13", "right,52px"),
        ("r-14", "right,56px"),
        ("r-15", "right,60px"),
        ("r-16", "right,64px"),
        ("r-17", "right,68px"),
        ("r-18", "right,72px"),
        ("r-19", "right,76px"),
        ("r-20", "right,80px"),
    ]);

    dwgenerate_map!("m", "margin-x-", [
        ("x-0", "0"),
        ("x-1", "4px"),
        ("x-2", "8px"),
        ("x-3", "12px"),
        ("x-4", "16px"),
        ("x-5", "20px"),
        ("x-6", "24px"),
        ("x-7", "28px"),
        ("x-8", "32px"),
        ("x-9", "36px"),
        ("x-10", "40px"),
        ("x-11", "44px"),
        ("x-12", "48px"),
        ("x-13", "52px"),
        ("x-14", "56px"),
        ("x-15", "60px"),
        ("x-16", "64px"),
        ("x-17", "68px"),
        ("x-18", "72px"),
        ("x-19", "76px"),
        ("x-20", "80px"),
    ]);
}

pub mod sizing {
    use dwind_macros::dwgenerate_map;

    include!(concat!(env!("OUT_DIR"), "/sizing.rs"));

    #[macro_export]
    macro_rules! width_generator {
        ($width:tt) => {
            const_format::formatcp!("width: {};", $width)
        };
    }

    dwgenerate_map!("w", "width-", [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
    ]);
}

pub mod color {
    use dwind_macros::{dwgenerate, dwgenerate_map};

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

    dwgenerate_map!("bg", "bg-color-", [
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
    ]);

    dwgenerate_map!("text", "text-color-", [
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
    ]);

    // Apple

    dwgenerate_map!("bg", "bg-color-", [
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
    ]);

    dwgenerate_map!("text", "text-color-", [
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
    ]);


    // Purple
    dwgenerate_map!("bg", "bg-color-", [
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
    ]);

    dwgenerate_map!("text", "text-color-", [
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
    ]);

    // Bermuda Gray
    dwgenerate_map!("bg", "bg-color-", [
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
    ]);

    dwgenerate_map!("text", "text-color-", [
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
    ]);

    // Candlelight
    dwgenerate_map!("bg", "bg-color-", [
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
    ]);

    dwgenerate_map!("bg-hover", "hover:bg-color-", [
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
    ]);
}