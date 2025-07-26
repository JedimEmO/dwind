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
macro_rules! margin_inline_start_generator {
    ($margin:tt) => {
        const_format::formatcp!("margin-inline-start: {};", $margin)
    };
}

#[macro_export]
macro_rules! margin_inline_end_generator {
    ($margin:tt) => {
        const_format::formatcp!("margin-inline-end: {};", $margin)
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
macro_rules! padding_inline_start_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding-inline-start: {};", $padding)
    };
}

#[macro_export]
macro_rules! padding_inline_end_generator {
    ($padding:tt) => {
        const_format::formatcp!("padding-inline-end: {};", $padding)
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
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("13", "3.25rem"),
        ("14", "3.5rem"),
        ("15", "3.75rem"),
        ("16", "4rem"),
        ("17", "4.25rem"),
        ("18", "4.5rem"),
        ("19", "4.75rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

dwgenerate_map!(
    "m",
    "margin-dir-",
    [
        ("l-0", "left,0"),
        ("l-1", "left,0.25rem"),
        ("l-2", "left,0.5rem"),
        ("l-3", "left,0.75rem"),
        ("l-4", "left,1rem"),
        ("l-5", "left,1.25rem"),
        ("l-6", "left,1.5rem"),
        ("l-7", "left,1.75rem"),
        ("l-8", "left,2rem"),
        ("l-9", "left,2.25rem"),
        ("l-10", "left,2.5rem"),
        ("l-11", "left,2.75rem"),
        ("l-12", "left,3rem"),
        ("l-13", "left,3.25rem"),
        ("l-14", "left,3.5rem"),
        ("l-15", "left,3.75rem"),
        ("l-16", "left,4rem"),
        ("l-17", "left,4.25rem"),
        ("l-18", "left,4.5rem"),
        ("l-19", "left,4.75rem"),
        ("l-20", "left,5rem"),
        ("r-0", "right,0"),
        ("r-1", "right,0.25rem"),
        ("r-2", "right,0.5rem"),
        ("r-3", "right,0.75rem"),
        ("r-4", "right,1rem"),
        ("r-5", "right,1.25rem"),
        ("r-6", "right,1.5rem"),
        ("r-7", "right,1.75rem"),
        ("r-8", "right,2rem"),
        ("r-9", "right,2.25rem"),
        ("r-10", "right,2.5rem"),
        ("r-11", "right,2.75rem"),
        ("r-12", "right,3rem"),
        ("r-13", "right,3.25rem"),
        ("r-14", "right,3.5rem"),
        ("r-15", "right,3.75rem"),
        ("r-16", "right,4rem"),
        ("r-17", "right,4.25rem"),
        ("r-18", "right,4.5rem"),
        ("r-19", "right,4.75rem"),
        ("r-20", "right,5rem"),
        ("t-0", "top,0"),
        ("t-1", "top,0.25rem"),
        ("t-2", "top,0.5rem"),
        ("t-3", "top,0.75rem"),
        ("t-4", "top,1rem"),
        ("t-5", "top,1.25rem"),
        ("t-6", "top,1.5rem"),
        ("t-7", "top,1.75rem"),
        ("t-8", "top,2rem"),
        ("t-9", "top,2.25rem"),
        ("t-10", "top,2.5rem"),
        ("t-11", "top,2.75rem"),
        ("t-12", "top,3rem"),
        ("t-13", "top,3.25rem"),
        ("t-14", "top,3.5rem"),
        ("t-15", "top,3.75rem"),
        ("t-16", "top,4rem"),
        ("t-17", "top,4.25rem"),
        ("t-18", "top,4.5rem"),
        ("t-19", "top,4.75rem"),
        ("t-20", "top,5rem"),
        ("b-0", "bottom,0"),
        ("b-1", "bottom,0.25rem"),
        ("b-2", "bottom,0.5rem"),
        ("b-3", "bottom,0.75rem"),
        ("b-4", "bottom,1rem"),
        ("b-5", "bottom,1.25rem"),
        ("b-6", "bottom,1.5rem"),
        ("b-7", "bottom,1.75rem"),
        ("b-8", "bottom,2rem"),
        ("b-9", "bottom,2.25rem"),
        ("b-10", "bottom,2.5rem"),
        ("b-11", "bottom,2.75rem"),
        ("b-12", "bottom,3rem"),
        ("b-13", "bottom,3.25rem"),
        ("b-14", "bottom,3.5rem"),
        ("b-15", "bottom,3.75rem"),
        ("b-16", "bottom,4rem"),
        ("b-17", "bottom,4.25rem"),
        ("b-18", "bottom,4.5rem"),
        ("b-19", "bottom,4.75rem"),
        ("b-20", "bottom,5rem")
    ]
);

dwgenerate_map!(
    "m",
    "margin-x-",
    [
        ("x-0", "0"),
        ("x-1", "0.25rem"),
        ("x-2", "0.5rem"),
        ("x-3", "0.75rem"),
        ("x-4", "1rem"),
        ("x-5", "1.25rem"),
        ("x-6", "1.5rem"),
        ("x-7", "1.75rem"),
        ("x-8", "2rem"),
        ("x-9", "2.25rem"),
        ("x-10", "2.5rem"),
        ("x-11", "2.75rem"),
        ("x-12", "3rem"),
        ("x-13", "3.25rem"),
        ("x-14", "3.5rem"),
        ("x-15", "3.75rem"),
        ("x-16", "4rem"),
        ("x-17", "4.25rem"),
        ("x-18", "4.5rem"),
        ("x-19", "4.75rem"),
        ("x-20", "5rem"),
    ]
);

dwgenerate_map!(
    "p",
    "padding-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("13", "3.25rem"),
        ("14", "3.5rem"),
        ("15", "3.75rem"),
        ("16", "4rem"),
        ("17", "4.25rem"),
        ("18", "4.5rem"),
        ("19", "4.75rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

dwgenerate_map!(
    "p",
    "padding-dir-",
    [
        ("l-0", "left,0"),
        ("l-1", "left,0.25rem"),
        ("l-2", "left,0.5rem"),
        ("l-3", "left,0.75rem"),
        ("l-4", "left,1rem"),
        ("l-5", "left,1.25rem"),
        ("l-6", "left,1.5rem"),
        ("l-7", "left,1.75rem"),
        ("l-8", "left,2rem"),
        ("l-9", "left,2.25rem"),
        ("l-10", "left,2.5rem"),
        ("l-11", "left,2.75rem"),
        ("l-12", "left,3rem"),
        ("l-13", "left,3.25rem"),
        ("l-14", "left,3.5rem"),
        ("l-15", "left,3.75rem"),
        ("l-16", "left,4rem"),
        ("l-17", "left,4.25rem"),
        ("l-18", "left,4.5rem"),
        ("l-19", "left,4.75rem"),
        ("l-20", "left,5rem"),
        ("r-0", "right,0"),
        ("r-1", "right,0.25rem"),
        ("r-2", "right,0.5rem"),
        ("r-3", "right,0.75rem"),
        ("r-4", "right,1rem"),
        ("r-5", "right,1.25rem"),
        ("r-6", "right,1.5rem"),
        ("r-7", "right,1.75rem"),
        ("r-8", "right,2rem"),
        ("r-9", "right,2.25rem"),
        ("r-10", "right,2.5rem"),
        ("r-11", "right,2.75rem"),
        ("r-12", "right,3rem"),
        ("r-13", "right,3.25rem"),
        ("r-14", "right,3.5rem"),
        ("r-15", "right,3.75rem"),
        ("r-16", "right,4rem"),
        ("r-17", "right,4.25rem"),
        ("r-18", "right,4.5rem"),
        ("r-19", "right,4.75rem"),
        ("r-20", "right,5rem"),
        ("t-0", "top,0"),
        ("t-1", "top,0.25rem"),
        ("t-2", "top,0.5rem"),
        ("t-3", "top,0.75rem"),
        ("t-4", "top,1rem"),
        ("t-5", "top,1.25rem"),
        ("t-6", "top,1.5rem"),
        ("t-7", "top,1.75rem"),
        ("t-8", "top,2rem"),
        ("t-9", "top,2.25rem"),
        ("t-10", "top,2.5rem"),
        ("t-11", "top,2.75rem"),
        ("t-12", "top,3rem"),
        ("t-13", "top,3.25rem"),
        ("t-14", "top,3.5rem"),
        ("t-15", "top,3.75rem"),
        ("t-16", "top,4rem"),
        ("t-17", "top,4.25rem"),
        ("t-18", "top,4.5rem"),
        ("t-19", "top,4.75rem"),
        ("t-20", "top,5rem"),
        ("b-0", "bottom,0"),
        ("b-1", "bottom,0.25rem"),
        ("b-2", "bottom,0.5rem"),
        ("b-3", "bottom,0.75rem"),
        ("b-4", "bottom,1rem"),
        ("b-5", "bottom,1.25rem"),
        ("b-6", "bottom,1.5rem"),
        ("b-7", "bottom,1.75rem"),
        ("b-8", "bottom,2rem"),
        ("b-9", "bottom,2.25rem"),
        ("b-10", "bottom,2.5rem"),
        ("b-11", "bottom,2.75rem"),
        ("b-12", "bottom,3rem"),
        ("b-13", "bottom,3.25rem"),
        ("b-14", "bottom,3.5rem"),
        ("b-15", "bottom,3.75rem"),
        ("b-16", "bottom,4rem"),
        ("b-17", "bottom,4.25rem"),
        ("b-18", "bottom,4.5rem"),
        ("b-19", "bottom,4.75rem"),
        ("b-20", "bottom,5rem"),
    ]
);

dwgenerate_map!(
    "gap",
    "gap-",
    [
        ("0", "0"),
        ("1", "0.25rem"),
        ("2", "0.5rem"),
        ("3", "0.75rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("13", "3.25rem"),
        ("14", "3.5rem"),
        ("15", "3.75rem"),
        ("16", "4rem"),
        ("17", "4.25rem"),
        ("18", "4.5rem"),
        ("19", "4.75rem"),
        ("20", "5rem"),
    ]
);

// Tailwind-style margin directional classes
dwgenerate_map!(
    "mt",
    "margin-dir-",
    [
        ("0", "top,0px"),
        ("px", "top,1px"),
        ("0.5", "top,0.125rem"),
        ("1", "top,0.25rem"),
        ("1.5", "top,0.375rem"),
        ("2", "top,0.5rem"),
        ("2.5", "top,0.625rem"),
        ("3", "top,0.75rem"),
        ("3.5", "top,0.875rem"),
        ("4", "top,1rem"),
        ("5", "top,1.25rem"),
        ("6", "top,1.5rem"),
        ("7", "top,1.75rem"),
        ("8", "top,2rem"),
        ("9", "top,2.25rem"),
        ("10", "top,2.5rem"),
        ("11", "top,2.75rem"),
        ("12", "top,3rem"),
        ("14", "top,3.5rem"),
        ("16", "top,4rem"),
        ("20", "top,5rem"),
        ("24", "top,6rem"),
        ("28", "top,7rem"),
        ("32", "top,8rem"),
        ("36", "top,9rem"),
        ("40", "top,10rem"),
        ("44", "top,11rem"),
        ("48", "top,12rem"),
        ("52", "top,13rem"),
        ("56", "top,14rem"),
        ("60", "top,15rem"),
        ("64", "top,16rem"),
        ("72", "top,18rem"),
        ("80", "top,20rem"),
        ("96", "top,24rem")
    ]
);

dwgenerate_map!(
    "mb",
    "margin-dir-",
    [
        ("0", "bottom,0px"),
        ("px", "bottom,1px"),
        ("0.5", "bottom,0.125rem"),
        ("1", "bottom,0.25rem"),
        ("1.5", "bottom,0.375rem"),
        ("2", "bottom,0.5rem"),
        ("2.5", "bottom,0.625rem"),
        ("3", "bottom,0.75rem"),
        ("3.5", "bottom,0.875rem"),
        ("4", "bottom,1rem"),
        ("5", "bottom,1.25rem"),
        ("6", "bottom,1.5rem"),
        ("7", "bottom,1.75rem"),
        ("8", "bottom,2rem"),
        ("9", "bottom,2.25rem"),
        ("10", "bottom,2.5rem"),
        ("11", "bottom,2.75rem"),
        ("12", "bottom,3rem"),
        ("14", "bottom,3.5rem"),
        ("16", "bottom,4rem"),
        ("20", "bottom,5rem"),
        ("24", "bottom,6rem"),
        ("28", "bottom,7rem"),
        ("32", "bottom,8rem"),
        ("36", "bottom,9rem"),
        ("40", "bottom,10rem"),
        ("44", "bottom,11rem"),
        ("48", "bottom,12rem"),
        ("52", "bottom,13rem"),
        ("56", "bottom,14rem"),
        ("60", "bottom,15rem"),
        ("64", "bottom,16rem"),
        ("72", "bottom,18rem"),
        ("80", "bottom,20rem"),
        ("96", "bottom,24rem")
    ]
);

dwgenerate_map!(
    "ml",
    "margin-dir-",
    [
        ("0", "left,0px"),
        ("px", "left,1px"),
        ("0.5", "left,0.125rem"),
        ("1", "left,0.25rem"),
        ("1.5", "left,0.375rem"),
        ("2", "left,0.5rem"),
        ("2.5", "left,0.625rem"),
        ("3", "left,0.75rem"),
        ("3.5", "left,0.875rem"),
        ("4", "left,1rem"),
        ("5", "left,1.25rem"),
        ("6", "left,1.5rem"),
        ("7", "left,1.75rem"),
        ("8", "left,2rem"),
        ("9", "left,2.25rem"),
        ("10", "left,2.5rem"),
        ("11", "left,2.75rem"),
        ("12", "left,3rem"),
        ("14", "left,3.5rem"),
        ("16", "left,4rem"),
        ("20", "left,5rem"),
        ("24", "left,6rem"),
        ("28", "left,7rem"),
        ("32", "left,8rem"),
        ("36", "left,9rem"),
        ("40", "left,10rem"),
        ("44", "left,11rem"),
        ("48", "left,12rem"),
        ("52", "left,13rem"),
        ("56", "left,14rem"),
        ("60", "left,15rem"),
        ("64", "left,16rem"),
        ("72", "left,18rem"),
        ("80", "left,20rem"),
        ("96", "left,24rem")
    ]
);

dwgenerate_map!(
    "mr",
    "margin-dir-",
    [
        ("0", "right,0px"),
        ("px", "right,1px"),
        ("0.5", "right,0.125rem"),
        ("1", "right,0.25rem"),
        ("1.5", "right,0.375rem"),
        ("2", "right,0.5rem"),
        ("2.5", "right,0.625rem"),
        ("3", "right,0.75rem"),
        ("3.5", "right,0.875rem"),
        ("4", "right,1rem"),
        ("5", "right,1.25rem"),
        ("6", "right,1.5rem"),
        ("7", "right,1.75rem"),
        ("8", "right,2rem"),
        ("9", "right,2.25rem"),
        ("10", "right,2.5rem"),
        ("11", "right,2.75rem"),
        ("12", "right,3rem"),
        ("14", "right,3.5rem"),
        ("16", "right,4rem"),
        ("20", "right,5rem"),
        ("24", "right,6rem"),
        ("28", "right,7rem"),
        ("32", "right,8rem"),
        ("36", "right,9rem"),
        ("40", "right,10rem"),
        ("44", "right,11rem"),
        ("48", "right,12rem"),
        ("52", "right,13rem"),
        ("56", "right,14rem"),
        ("60", "right,15rem"),
        ("64", "right,16rem"),
        ("72", "right,18rem"),
        ("80", "right,20rem"),
        ("96", "right,24rem")
    ]
);

// Margin inline start classes
dwgenerate_map!(
    "ms",
    "margin-inline-start-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

// Margin inline end classes
dwgenerate_map!(
    "me",
    "margin-inline-end-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

// Tailwind-style margin axis classes
dwgenerate_map!(
    "mx",
    "margin-x-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

dwgenerate_map!(
    "my",
    "margin-y-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

// Tailwind-style padding directional classes
dwgenerate_map!(
    "pt",
    "padding-dir-",
    [
        ("0", "top,0px"),
        ("px", "top,1px"),
        ("0.5", "top,0.125rem"),
        ("1", "top,0.25rem"),
        ("1.5", "top,0.375rem"),
        ("2", "top,0.5rem"),
        ("2.5", "top,0.625rem"),
        ("3", "top,0.75rem"),
        ("3.5", "top,0.875rem"),
        ("4", "top,1rem"),
        ("5", "top,1.25rem"),
        ("6", "top,1.5rem"),
        ("7", "top,1.75rem"),
        ("8", "top,2rem"),
        ("9", "top,2.25rem"),
        ("10", "top,2.5rem"),
        ("11", "top,2.75rem"),
        ("12", "top,3rem"),
        ("13", "top,52px"),
        ("14", "top,3.5rem"),
        ("15", "top,60px"),
        ("16", "top,4rem"),
        ("17", "top,68px"),
        ("18", "top,72px"),
        ("19", "top,76px"),
        ("20", "top,5rem"),
        ("24", "top,6rem"),
        ("28", "top,7rem"),
        ("32", "top,8rem"),
        ("36", "top,9rem"),
        ("40", "top,10rem"),
        ("44", "top,11rem"),
        ("48", "top,12rem"),
        ("52", "top,13rem"),
        ("56", "top,14rem"),
        ("60", "top,15rem"),
        ("64", "top,16rem"),
        ("72", "top,18rem"),
        ("80", "top,20rem"),
        ("96", "top,24rem")
    ]
);

dwgenerate_map!(
    "pb",
    "padding-dir-",
    [
        ("0", "bottom,0px"),
        ("px", "bottom,1px"),
        ("0.5", "bottom,0.125rem"),
        ("1", "bottom,0.25rem"),
        ("1.5", "bottom,0.375rem"),
        ("2", "bottom,0.5rem"),
        ("2.5", "bottom,0.625rem"),
        ("3", "bottom,0.75rem"),
        ("3.5", "bottom,0.875rem"),
        ("4", "bottom,1rem"),
        ("5", "bottom,1.25rem"),
        ("6", "bottom,1.5rem"),
        ("7", "bottom,1.75rem"),
        ("8", "bottom,2rem"),
        ("9", "bottom,2.25rem"),
        ("10", "bottom,2.5rem"),
        ("11", "bottom,2.75rem"),
        ("12", "bottom,3rem"),
        ("13", "bottom,52px"),
        ("14", "bottom,3.5rem"),
        ("15", "bottom,60px"),
        ("16", "bottom,4rem"),
        ("17", "bottom,68px"),
        ("18", "bottom,72px"),
        ("19", "bottom,76px"),
        ("20", "bottom,5rem"),
        ("24", "bottom,6rem"),
        ("28", "bottom,7rem"),
        ("32", "bottom,8rem"),
        ("36", "bottom,9rem"),
        ("40", "bottom,10rem"),
        ("44", "bottom,11rem"),
        ("48", "bottom,12rem"),
        ("52", "bottom,13rem"),
        ("56", "bottom,14rem"),
        ("60", "bottom,15rem"),
        ("64", "bottom,16rem"),
        ("72", "bottom,18rem"),
        ("80", "bottom,20rem"),
        ("96", "bottom,24rem")
    ]
);

dwgenerate_map!(
    "pl",
    "padding-dir-",
    [
        ("0", "left,0px"),
        ("px", "left,1px"),
        ("0.5", "left,0.125rem"),
        ("1", "left,0.25rem"),
        ("1.5", "left,0.375rem"),
        ("2", "left,0.5rem"),
        ("2.5", "left,0.625rem"),
        ("3", "left,0.75rem"),
        ("3.5", "left,0.875rem"),
        ("4", "left,1rem"),
        ("5", "left,1.25rem"),
        ("6", "left,1.5rem"),
        ("7", "left,1.75rem"),
        ("8", "left,2rem"),
        ("9", "left,2.25rem"),
        ("10", "left,2.5rem"),
        ("11", "left,2.75rem"),
        ("12", "left,3rem"),
        ("13", "left,52px"),
        ("14", "left,3.5rem"),
        ("15", "left,60px"),
        ("16", "left,4rem"),
        ("17", "left,68px"),
        ("18", "left,72px"),
        ("19", "left,76px"),
        ("20", "left,5rem"),
        ("24", "left,6rem"),
        ("28", "left,7rem"),
        ("32", "left,8rem"),
        ("36", "left,9rem"),
        ("40", "left,10rem"),
        ("44", "left,11rem"),
        ("48", "left,12rem"),
        ("52", "left,13rem"),
        ("56", "left,14rem"),
        ("60", "left,15rem"),
        ("64", "left,16rem"),
        ("72", "left,18rem"),
        ("80", "left,20rem"),
        ("96", "left,24rem")
    ]
);

dwgenerate_map!(
    "pr",
    "padding-dir-",
    [
        ("0", "right,0px"),
        ("px", "right,1px"),
        ("0.5", "right,0.125rem"),
        ("1", "right,0.25rem"),
        ("1.5", "right,0.375rem"),
        ("2", "right,0.5rem"),
        ("2.5", "right,0.625rem"),
        ("3", "right,0.75rem"),
        ("3.5", "right,0.875rem"),
        ("4", "right,1rem"),
        ("5", "right,1.25rem"),
        ("6", "right,1.5rem"),
        ("7", "right,1.75rem"),
        ("8", "right,2rem"),
        ("9", "right,2.25rem"),
        ("10", "right,2.5rem"),
        ("11", "right,2.75rem"),
        ("12", "right,3rem"),
        ("13", "right,52px"),
        ("14", "right,3.5rem"),
        ("15", "right,60px"),
        ("16", "right,4rem"),
        ("17", "right,68px"),
        ("18", "right,72px"),
        ("19", "right,76px"),
        ("20", "right,5rem"),
        ("24", "right,6rem"),
        ("28", "right,7rem"),
        ("32", "right,8rem"),
        ("36", "right,9rem"),
        ("40", "right,10rem"),
        ("44", "right,11rem"),
        ("48", "right,12rem"),
        ("52", "right,13rem"),
        ("56", "right,14rem"),
        ("60", "right,15rem"),
        ("64", "right,16rem"),
        ("72", "right,18rem"),
        ("80", "right,20rem"),
        ("96", "right,24rem")
    ]
);

// Tailwind-style padding axis classes
dwgenerate_map!(
    "px",
    "padding-x-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("13", "3.25rem"),
        ("14", "3.5rem"),
        ("15", "3.75rem"),
        ("16", "4rem"),
        ("17", "4.25rem"),
        ("18", "4.5rem"),
        ("19", "4.75rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

dwgenerate_map!(
    "py",
    "padding-y-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("13", "3.25rem"),
        ("14", "3.5rem"),
        ("15", "3.75rem"),
        ("16", "4rem"),
        ("17", "4.25rem"),
        ("18", "4.5rem"),
        ("19", "4.75rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

// Padding inline start classes
dwgenerate_map!(
    "ps",
    "padding-inline-start-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);

// Padding inline end classes
dwgenerate_map!(
    "pe",
    "padding-inline-end-",
    [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem")
    ]
);