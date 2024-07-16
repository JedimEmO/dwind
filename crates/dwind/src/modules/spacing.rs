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


dwgenerate_map!(
    "m",
    "margin-",
    [
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
        ("t-0", "top,0"),
        ("t-1", "top,4px"),
        ("t-2", "top,8px"),
        ("t-3", "top,12px"),
        ("t-4", "top,16px"),
        ("t-5", "top,20px"),
        ("t-6", "top,24px"),
        ("t-7", "top,28px"),
        ("t-8", "top,32px"),
        ("t-9", "top,36px"),
        ("t-10", "top,40px"),
        ("t-11", "top,44px"),
        ("t-12", "top,48px"),
        ("t-13", "top,52px"),
        ("t-14", "top,56px"),
        ("t-15", "top,60px"),
        ("t-16", "top,64px"),
        ("t-17", "top,68px"),
        ("t-18", "top,72px"),
        ("t-19", "top,76px"),
        ("t-20", "top,80px"),
        ("b-0", "bottom,0"),
        ("b-1", "bottom,4px"),
        ("b-2", "bottom,8px"),
        ("b-3", "bottom,12px"),
        ("b-4", "bottom,16px"),
        ("b-5", "bottom,20px"),
        ("b-6", "bottom,24px"),
        ("b-7", "bottom,28px"),
        ("b-8", "bottom,32px"),
        ("b-9", "bottom,36px"),
        ("b-10", "bottom,40px"),
        ("b-11", "bottom,44px"),
        ("b-12", "bottom,48px"),
        ("b-13", "bottom,52px"),
        ("b-14", "bottom,56px"),
        ("b-15", "bottom,60px"),
        ("b-16", "bottom,64px"),
        ("b-17", "bottom,68px"),
        ("b-18", "bottom,72px"),
        ("b-19", "bottom,76px"),
        ("b-20", "bottom,80px"),
    ]
);

dwgenerate_map!(
    "m",
    "margin-x-",
    [
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
    ]
);
