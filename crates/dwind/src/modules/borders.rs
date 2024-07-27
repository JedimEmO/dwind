use dwind_macros::dwgenerate_map;

include!(concat!(env!("OUT_DIR"), "/borders.rs"));

#[macro_export]
macro_rules! border_width_generator {
    ($width:tt) => {
        const_format::formatcp!("border-width: {};", $width)
    };
}

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
