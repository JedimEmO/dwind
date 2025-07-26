use dwind_macros::dwgenerate_map;

#[macro_export]
macro_rules! aspect_generator {
    ($color:tt) => {
        const_format::formatcp!("aspect-ratio: {};", $color)
    };
}

#[macro_export]
macro_rules! inset_generator {
    ($value:tt) => {
        const_format::formatcp!("inset: {};", $value)
    };
}

#[macro_export]
macro_rules! inset_x_generator {
    ($value:tt) => {
        const_format::formatcp!("left: {};right: {};", $value, $value)
    };
}

#[macro_export]
macro_rules! inset_y_generator {
    ($value:tt) => {
        const_format::formatcp!("top: {};bottom: {};", $value, $value)
    };
}

#[macro_export]
macro_rules! inset_inline_start_generator {
    ($value:tt) => {
        const_format::formatcp!("inset-inline-start: {};", $value)
    };
}

#[macro_export]
macro_rules! inset_inline_end_generator {
    ($value:tt) => {
        const_format::formatcp!("inset-inline-end: {};", $value)
    };
}

#[macro_export]
macro_rules! position_generator {
    ($prop:tt, $value:tt) => {
        const_format::formatcp!("{}: {};", $prop, $value)
    };
}

include!(concat!(env!("OUT_DIR"), "/layout.rs"));

// Inset all sides
dwgenerate_map!(
    "inset",
    "inset-",
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
        ("96", "24rem"),
        ("auto", "auto"),
        ("1/2", "50%"),
        ("1/3", "33.333333%"),
        ("2/3", "66.666667%"),
        ("1/4", "25%"),
        ("2/4", "50%"),
        ("3/4", "75%"),
        ("full", "100%")
    ]
);

// Inset X (left/right)
dwgenerate_map!(
    "inset-x",
    "inset-x-",
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
        ("96", "24rem"),
        ("auto", "auto"),
        ("1/2", "50%"),
        ("1/3", "33.333333%"),
        ("2/3", "66.666667%"),
        ("1/4", "25%"),
        ("2/4", "50%"),
        ("3/4", "75%"),
        ("full", "100%")
    ]
);

// Inset Y (top/bottom)
dwgenerate_map!(
    "inset-y",
    "inset-y-",
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
        ("96", "24rem"),
        ("auto", "auto"),
        ("1/2", "50%"),
        ("1/3", "33.333333%"),
        ("2/3", "66.666667%"),
        ("1/4", "25%"),
        ("2/4", "50%"),
        ("3/4", "75%"),
        ("full", "100%")
    ]
);

// Start (inset-inline-start)
dwgenerate_map!(
    "start",
    "inset-inline-start-",
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
        ("96", "24rem"),
        ("auto", "auto"),
        ("1/2", "50%"),
        ("1/3", "33.333333%"),
        ("2/3", "66.666667%"),
        ("1/4", "25%"),
        ("2/4", "50%"),
        ("3/4", "75%"),
        ("full", "100%")
    ]
);

// End (inset-inline-end)
dwgenerate_map!(
    "end",
    "inset-inline-end-",
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
        ("96", "24rem"),
        ("auto", "auto"),
        ("1/2", "50%"),
        ("1/3", "33.333333%"),
        ("2/3", "66.666667%"),
        ("1/4", "25%"),
        ("2/4", "50%"),
        ("3/4", "75%"),
        ("full", "100%")
    ]
);

// Top
dwgenerate_map!(
    "top",
    "position-",
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
        ("96", "top,24rem"),
        ("auto", "top,auto"),
        ("1/2", "top,50%"),
        ("1/3", "top,33.333333%"),
        ("2/3", "top,66.666667%"),
        ("1/4", "top,25%"),
        ("2/4", "top,50%"),
        ("3/4", "top,75%"),
        ("full", "top,100%")
    ]
);

// Right
dwgenerate_map!(
    "right",
    "position-",
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
        ("96", "right,24rem"),
        ("auto", "right,auto"),
        ("1/2", "right,50%"),
        ("1/3", "right,33.333333%"),
        ("2/3", "right,66.666667%"),
        ("1/4", "right,25%"),
        ("2/4", "right,50%"),
        ("3/4", "right,75%"),
        ("full", "right,100%")
    ]
);

// Bottom
dwgenerate_map!(
    "bottom",
    "position-",
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
        ("96", "bottom,24rem"),
        ("auto", "bottom,auto"),
        ("1/2", "bottom,50%"),
        ("1/3", "bottom,33.333333%"),
        ("2/3", "bottom,66.666667%"),
        ("1/4", "bottom,25%"),
        ("2/4", "bottom,50%"),
        ("3/4", "bottom,75%"),
        ("full", "bottom,100%")
    ]
);

// Left
dwgenerate_map!(
    "left",
    "position-",
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
        ("96", "left,24rem"),
        ("auto", "left,auto"),
        ("1/2", "left,50%"),
        ("1/3", "left,33.333333%"),
        ("2/3", "left,66.666667%"),
        ("1/4", "left,25%"),
        ("2/4", "left,50%"),
        ("3/4", "left,75%"),
        ("full", "left,100%")
    ]
);
