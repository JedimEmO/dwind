mod modules;

pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub use modules::borders;

pub mod interactivity {
    include!(concat!(env!("OUT_DIR"), "/interactivity.rs"));
}

pub mod flexbox_and_grid {
    include!(concat!(env!("OUT_DIR"), "/flexbox_and_grid.rs"));
}

pub mod typography {
    include!(concat!(env!("OUT_DIR"), "/typography.rs"));
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

    #[macro_export]
    macro_rules! max_width_generator {
        ($width:tt) => {
            const_format::formatcp!("max-width: {};", $width)
        };
    }

    dwgenerate_map!(
        "w",
        "width-",
        [
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
            ("24", "96px"),
            ("28", "112px"),
            ("32", "128px"),
            ("36", "144px"),
            ("40", "160px"),
            ("44", "176px"),
            ("48", "192px"),
            ("52", "208px"),
            ("56", "224px"),
            ("60", "240px"),
            ("64", "256px"),
            ("72", "288px"),
            ("80", "320px"),
            ("96", "384px"),
            ("sm", "384px"),
            ("md", "768px"),
            ("lg", "1024px"),
            ("xl", "1280px"),
            ("2xl", "1536px"),
            ("3xl", "1792px"),
            ("4xl", "2048px"),
            ("5xl", "2304px"),
            ("6xl", "2560px"),
            ("7xl", "2816px"),
            ("8xl", "3072px"),
            ("9xl", "3328px"),
            ("10xl", "3584px"),
            ("px", "1px"),
            ("0", "0"),
            ("auto", "auto"),
            ("screen", "100vw"),
            ("full", "100%"),
            ("min", "min-content"),
            ("max", "max-content"),
        ]
    );

    dwgenerate_map!(
        "max-w",
        "max-width-",
        [
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
            ("24", "96px"),
            ("28", "112px"),
            ("32", "128px"),
            ("36", "144px"),
            ("40", "160px"),
            ("44", "176px"),
            ("48", "192px"),
            ("52", "208px"),
            ("56", "224px"),
            ("60", "240px"),
            ("64", "256px"),
            ("72", "288px"),
            ("80", "320px"),
            ("96", "384px"),
            ("sm", "384px"),
            ("md", "768px"),
            ("lg", "1024px"),
            ("xl", "1280px"),
            ("2xl", "1536px"),
            ("3xl", "1792px"),
            ("4xl", "2048px"),
            ("5xl", "2304px"),
            ("6xl", "2560px"),
            ("7xl", "2816px"),
            ("8xl", "3072px"),
            ("9xl", "3328px"),
            ("10xl", "3584px"),
            ("px", "1px"),
            ("0", "0"),
            ("auto", "auto"),
            ("screen", "100vw"),
            ("full", "100%"),
            ("min", "min-content"),
            ("max", "max-content"),
        ]
    );
}

pub use modules::colors;
