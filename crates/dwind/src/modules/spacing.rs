use dwind_macros::dwgenerate_map;

include!(concat!(env!("OUT_DIR"), "/spacing.rs"));

#[macro_export]
macro_rules! margin_x_generator {
    ($margin:tt) => {
        const_format::formatcp!("margin-left: {};margin-right: {};", $margin, $margin)
    };
}

#[macro_export]
macro_rules! margin_y_generator {
    ($margin:tt) => {
        const_format::formatcp!("margin-top: {};margin-bottom: {};", $margin, $margin)
    };
}

#[macro_export]
macro_rules! margin_dir_generator {
    ($dir:tt, $margin:tt) => {
        const_format::formatcp!("margin-{}: {};", $dir, $margin)
    };
}

#[macro_export]
macro_rules! margin_generator {
    ($margin:tt) => {
        const_format::formatcp!("margin: {};", $margin)
    };
}

#[macro_export]
macro_rules! padding_dir_generator {
    ($dir:tt, $padding:tt) => {
        const_format::formatcp!("padding-{}: {};", $dir, $padding)
    };
}

#[macro_export]
macro_rules! padding_x_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding-left: {};padding-right: {};", $padding, $padding)
    };
}

#[macro_export]
macro_rules! padding_y_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding-top: {};padding-bottom: {};", $padding, $padding)
    };
}

#[macro_export]
macro_rules! padding_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding: {};", $padding)
    };
}

#[macro_export]
macro_rules! gap_generator {
    ($gap:tt) => {
        const_format::formatcp!("gap: {};", $gap)
    };
}

dwgenerate_map!(
    "m",
    "margin-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px")
    ]
);

dwgenerate_map!(
    "m",
    "margin-dir-",
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
        ("b-20", "bottom,80px")
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

dwgenerate_map!(
    "p",
    "padding-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);

dwgenerate_map!(
    "p",
    "padding-dir-",
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
    "gap",
    "gap-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);

// Tailwind-style margin directional classes
dwgenerate_map!(
    "mt",
    "margin-dir-",
    [
        ("0", "top,0"),
        ("1", "top,4px"),
        ("2", "top,8px"),
        ("3", "top,12px"),
        ("4", "top,16px"),
        ("5", "top,20px"),
        ("6", "top,24px"),
        ("7", "top,28px"),
        ("8", "top,32px"),
        ("9", "top,36px"),
        ("10", "top,40px"),
        ("11", "top,44px"),
        ("12", "top,48px"),
        ("13", "top,52px"),
        ("14", "top,56px"),
        ("15", "top,60px"),
        ("16", "top,64px"),
        ("17", "top,68px"),
        ("18", "top,72px"),
        ("19", "top,76px"),
        ("20", "top,80px"),
    ]
);

dwgenerate_map!(
    "mb",
    "margin-dir-",
    [
        ("0", "bottom,0"),
        ("1", "bottom,4px"),
        ("2", "bottom,8px"),
        ("3", "bottom,12px"),
        ("4", "bottom,16px"),
        ("5", "bottom,20px"),
        ("6", "bottom,24px"),
        ("7", "bottom,28px"),
        ("8", "bottom,32px"),
        ("9", "bottom,36px"),
        ("10", "bottom,40px"),
        ("11", "bottom,44px"),
        ("12", "bottom,48px"),
        ("13", "bottom,52px"),
        ("14", "bottom,56px"),
        ("15", "bottom,60px"),
        ("16", "bottom,64px"),
        ("17", "bottom,68px"),
        ("18", "bottom,72px"),
        ("19", "bottom,76px"),
        ("20", "bottom,80px"),
    ]
);

dwgenerate_map!(
    "ml",
    "margin-dir-",
    [
        ("0", "left,0"),
        ("1", "left,4px"),
        ("2", "left,8px"),
        ("3", "left,12px"),
        ("4", "left,16px"),
        ("5", "left,20px"),
        ("6", "left,24px"),
        ("7", "left,28px"),
        ("8", "left,32px"),
        ("9", "left,36px"),
        ("10", "left,40px"),
        ("11", "left,44px"),
        ("12", "left,48px"),
        ("13", "left,52px"),
        ("14", "left,56px"),
        ("15", "left,60px"),
        ("16", "left,64px"),
        ("17", "left,68px"),
        ("18", "left,72px"),
        ("19", "left,76px"),
        ("20", "left,80px"),
    ]
);

dwgenerate_map!(
    "mr",
    "margin-dir-",
    [
        ("0", "right,0"),
        ("1", "right,4px"),
        ("2", "right,8px"),
        ("3", "right,12px"),
        ("4", "right,16px"),
        ("5", "right,20px"),
        ("6", "right,24px"),
        ("7", "right,28px"),
        ("8", "right,32px"),
        ("9", "right,36px"),
        ("10", "right,40px"),
        ("11", "right,44px"),
        ("12", "right,48px"),
        ("13", "right,52px"),
        ("14", "right,56px"),
        ("15", "right,60px"),
        ("16", "right,64px"),
        ("17", "right,68px"),
        ("18", "right,72px"),
        ("19", "right,76px"),
        ("20", "right,80px"),
    ]
);

// Tailwind-style margin axis classes
dwgenerate_map!(
    "mx",
    "margin-x-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);

dwgenerate_map!(
    "my",
    "margin-y-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);

// Tailwind-style padding directional classes
dwgenerate_map!(
    "pt",
    "padding-dir-",
    [
        ("0", "top,0"),
        ("1", "top,4px"),
        ("2", "top,8px"),
        ("3", "top,12px"),
        ("4", "top,16px"),
        ("5", "top,20px"),
        ("6", "top,24px"),
        ("7", "top,28px"),
        ("8", "top,32px"),
        ("9", "top,36px"),
        ("10", "top,40px"),
        ("11", "top,44px"),
        ("12", "top,48px"),
        ("13", "top,52px"),
        ("14", "top,56px"),
        ("15", "top,60px"),
        ("16", "top,64px"),
        ("17", "top,68px"),
        ("18", "top,72px"),
        ("19", "top,76px"),
        ("20", "top,80px"),
    ]
);

dwgenerate_map!(
    "pb",
    "padding-dir-",
    [
        ("0", "bottom,0"),
        ("1", "bottom,4px"),
        ("2", "bottom,8px"),
        ("3", "bottom,12px"),
        ("4", "bottom,16px"),
        ("5", "bottom,20px"),
        ("6", "bottom,24px"),
        ("7", "bottom,28px"),
        ("8", "bottom,32px"),
        ("9", "bottom,36px"),
        ("10", "bottom,40px"),
        ("11", "bottom,44px"),
        ("12", "bottom,48px"),
        ("13", "bottom,52px"),
        ("14", "bottom,56px"),
        ("15", "bottom,60px"),
        ("16", "bottom,64px"),
        ("17", "bottom,68px"),
        ("18", "bottom,72px"),
        ("19", "bottom,76px"),
        ("20", "bottom,80px"),
    ]
);

dwgenerate_map!(
    "pl",
    "padding-dir-",
    [
        ("0", "left,0"),
        ("1", "left,4px"),
        ("2", "left,8px"),
        ("3", "left,12px"),
        ("4", "left,16px"),
        ("5", "left,20px"),
        ("6", "left,24px"),
        ("7", "left,28px"),
        ("8", "left,32px"),
        ("9", "left,36px"),
        ("10", "left,40px"),
        ("11", "left,44px"),
        ("12", "left,48px"),
        ("13", "left,52px"),
        ("14", "left,56px"),
        ("15", "left,60px"),
        ("16", "left,64px"),
        ("17", "left,68px"),
        ("18", "left,72px"),
        ("19", "left,76px"),
        ("20", "left,80px"),
    ]
);

dwgenerate_map!(
    "pr",
    "padding-dir-",
    [
        ("0", "right,0"),
        ("1", "right,4px"),
        ("2", "right,8px"),
        ("3", "right,12px"),
        ("4", "right,16px"),
        ("5", "right,20px"),
        ("6", "right,24px"),
        ("7", "right,28px"),
        ("8", "right,32px"),
        ("9", "right,36px"),
        ("10", "right,40px"),
        ("11", "right,44px"),
        ("12", "right,48px"),
        ("13", "right,52px"),
        ("14", "right,56px"),
        ("15", "right,60px"),
        ("16", "right,64px"),
        ("17", "right,68px"),
        ("18", "right,72px"),
        ("19", "right,76px"),
        ("20", "right,80px"),
    ]
);

// Tailwind-style padding axis classes
dwgenerate_map!(
    "px",
    "padding-x-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);

dwgenerate_map!(
    "py",
    "padding-y-",
    [
        ("0", "0"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
    ]
);
