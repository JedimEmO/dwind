use dwind_macros::dwgenerate_map;

include!(concat!(env!("OUT_DIR"), "/borders.rs"));

#[macro_export]
macro_rules! border_width_generator {
    ($width:tt) => {
        const_format::formatcp!("border-width: {};", $width)
    };
}

#[macro_export]
macro_rules! border_x_generator {
    ($width:tt) => {
        const_format::formatcp!(
            "border-left-width: {};border-right-width: {};",
            $width,
            $width
        )
    };
}

#[macro_export]
macro_rules! border_y_generator {
    ($width:tt) => {
        const_format::formatcp!(
            "border-top-width: {};border-bottom-width: {};",
            $width,
            $width
        )
    };
}

#[macro_export]
macro_rules! border_dir_generator {
    ($dir:tt, $width:tt) => {
        const_format::formatcp!("border-{}-width: {};", $dir, $width)
    };
}

#[macro_export]
macro_rules! border_s_generator {
    ($width:tt) => {
        const_format::formatcp!("border-inline-start-width: {};", $width)
    };
}

#[macro_export]
macro_rules! border_e_generator {
    ($width:tt) => {
        const_format::formatcp!("border-inline-end-width: {};", $width)
    };
}

// Border width classes (border-0, border-2, border-4, border-8, border)
dwgenerate_map!(
    "border",
    "border-width-",
    [("0", "0px"), ("2", "2px"), ("4", "4px"), ("8", "8px"),]
);

// Border X-axis classes (border-x-0, border-x-2, border-x-4, border-x-8, border-x)
dwgenerate_map!(
    "border",
    "border-x-",
    [
        ("x-0", "0px"),
        ("x-2", "2px"),
        ("x-4", "4px"),
        ("x-8", "8px"),
        ("x", "1px")
    ]
);

// Border Y-axis classes (border-y-0, border-y-2, border-y-4, border-y-8, border-y)
dwgenerate_map!(
    "border",
    "border-y-",
    [
        ("y-0", "0px"),
        ("y-2", "2px"),
        ("y-4", "4px"),
        ("y-8", "8px"),
        ("y", "1px")
    ]
);

// Border start classes (border-s-0, border-s-2, border-s-4, border-s-8, border-s)
dwgenerate_map!(
    "border",
    "border-s-",
    [
        ("s-0", "0px"),
        ("s-2", "2px"),
        ("s-4", "4px"),
        ("s-8", "8px"),
        ("s", "1px")
    ]
);

// Border end classes (border-e-0, border-e-2, border-e-4, border-e-8, border-e)
dwgenerate_map!(
    "border",
    "border-e-",
    [
        ("e-0", "0px"),
        ("e-2", "2px"),
        ("e-4", "4px"),
        ("e-8", "8px"),
        ("e", "1px")
    ]
);

// Border directional classes (border-t-*, border-r-*, border-b-*, border-l-*)
dwgenerate_map!(
    "border",
    "border-dir-",
    [
        ("t-0", "top,0px"),
        ("t-2", "top,2px"),
        ("t-4", "top,4px"),
        ("t-8", "top,8px"),
        ("t", "top,1px"),
        ("r-0", "right,0px"),
        ("r-2", "right,2px"),
        ("r-4", "right,4px"),
        ("r-8", "right,8px"),
        ("r", "right,1px"),
        ("b-0", "bottom,0px"),
        ("b-2", "bottom,2px"),
        ("b-4", "bottom,4px"),
        ("b-8", "bottom,8px"),
        ("b", "bottom,1px"),
        ("l-0", "left,0px"),
        ("l-2", "left,2px"),
        ("l-4", "left,4px"),
        ("l-8", "left,8px"),
        ("l", "left,1px")
    ]
);

// Keep the original border-w- mapping for backwards compatibility
dwgenerate_map!(
    "border-w",
    "border-width-",
    [
        ("0", "0"),
        ("1px", "1px"),
        ("2px", "2px"),
        ("3px", "3px"),
        ("4px", "4px"),
        ("5px", "5px"),
        ("6px", "6px"),
        ("7px", "7px"),
        ("8px", "8px"),
        ("9px", "9px"),
        ("10px", "10px"),
        ("11px", "11px"),
        ("12px", "12px"),
        ("13px", "13px"),
        ("14px", "14px"),
        ("15px", "15px"),
        ("16px", "16px"),
        ("17px", "17px"),
        ("18px", "18px"),
        ("19px", "19px"),
        ("20px", "20px"),
        ("21px", "21px"),
        ("22px", "22px"),
        ("23px", "23px"),
        ("24px", "24px"),
        ("25px", "25px"),
        ("26px", "26px"),
        ("27px", "27px"),
        ("28px", "28px"),
        ("29px", "29px"),
        ("30px", "30px"),
        ("31px", "31px"),
        ("32px", "32px"),
        ("33px", "33px"),
        ("34px", "34px"),
        ("35px", "35px"),
        ("36px", "36px"),
        ("37px", "37px"),
        ("38px", "38px"),
        ("39px", "39px"),
        ("40px", "40px"),
        ("41px", "41px"),
        ("42px", "42px"),
        ("43px", "43px"),
        ("44px", "44px"),
        ("45px", "45px"),
        ("46px", "46px"),
        ("47px", "47px"),
        ("48px", "48px"),
        ("49px", "49px"),
        ("50px", "50px"),
        ("51px", "51px"),
        ("52px", "52px"),
        ("53px", "53px"),
        ("54px", "54px"),
        ("55px", "55px"),
        ("56px", "56px")
    ]
);
