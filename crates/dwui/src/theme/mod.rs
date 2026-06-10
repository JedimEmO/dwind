use dominator::stylesheet;

const DWUI_KEYFRAMES: &str = r#"
@keyframes dwui-modal-in {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(0.5rem);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

@keyframes dwui-fade-in {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@keyframes dwui-progress-indeterminate {
    0% {
        margin-left: -40%;
    }
    100% {
        margin-left: 100%;
    }
}

@keyframes dwui-spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

@keyframes dwui-skeleton-pulse {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0.45;
    }
}"#;

pub fn apply_style_sheet(colors: Option<crate::theme::colors::ColorsCssVariables>) {
    stylesheet!(":root", {
        .raw(colors.unwrap_or_default().to_style_sheet_raw())
    });

    dominator::stylesheet_raw(DWUI_KEYFRAMES);

    base::apply_base_stylesheet();
    colors::apply_colors_stylesheet();
}

pub mod prelude {
    pub use super::base::*;
    pub use super::colors::*;
}

pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub mod colors {
    use std::collections::BTreeMap;

    include!(concat!(env!("OUT_DIR"), "/colors.rs"));

    impl ColorsCssVariables {
        /// Builds the theme variables from full shade palettes.
        ///
        /// `success` and `warning` default to sensible dwind palettes
        /// (apple/candlelight) when not provided; use [`Self::with_status_colors`]
        /// to override them.
        pub fn new(
            primary: &BTreeMap<u32, String>,
            text_on_primary: &BTreeMap<u32, String>,
            bg_void: &BTreeMap<u32, String>,
            error: &BTreeMap<u32, String>,
        ) -> Self {
            Self::with_status_colors(
                primary,
                text_on_primary,
                bg_void,
                error,
                &default_success_palette(),
                &default_warning_palette(),
            )
        }

        /// Like [`Self::new`], but with explicit success and warning palettes.
        pub fn with_status_colors(
            primary: &BTreeMap<u32, String>,
            text_on_primary: &BTreeMap<u32, String>,
            bg_void: &BTreeMap<u32, String>,
            error: &BTreeMap<u32, String>,
            success: &BTreeMap<u32, String>,
            warning: &BTreeMap<u32, String>,
        ) -> Self {
            Self {
                dwui_success_50: success.get(&50).unwrap().clone(),
                dwui_success_100: success.get(&100).unwrap().clone(),
                dwui_success_200: success.get(&200).unwrap().clone(),
                dwui_success_300: success.get(&300).unwrap().clone(),
                dwui_success_400: success.get(&400).unwrap().clone(),
                dwui_success_500: success.get(&500).unwrap().clone(),
                dwui_success_600: success.get(&600).unwrap().clone(),
                dwui_success_700: success.get(&700).unwrap().clone(),
                dwui_success_800: success.get(&800).unwrap().clone(),
                dwui_success_900: success.get(&900).unwrap().clone(),
                dwui_success_950: success.get(&950).unwrap().clone(),

                dwui_warning_50: warning.get(&50).unwrap().clone(),
                dwui_warning_100: warning.get(&100).unwrap().clone(),
                dwui_warning_200: warning.get(&200).unwrap().clone(),
                dwui_warning_300: warning.get(&300).unwrap().clone(),
                dwui_warning_400: warning.get(&400).unwrap().clone(),
                dwui_warning_500: warning.get(&500).unwrap().clone(),
                dwui_warning_600: warning.get(&600).unwrap().clone(),
                dwui_warning_700: warning.get(&700).unwrap().clone(),
                dwui_warning_800: warning.get(&800).unwrap().clone(),
                dwui_warning_900: warning.get(&900).unwrap().clone(),
                dwui_warning_950: warning.get(&950).unwrap().clone(),
                dwui_primary_50: primary.get(&50).unwrap().clone(),
                dwui_primary_100: primary.get(&100).unwrap().clone(),
                dwui_primary_200: primary.get(&200).unwrap().clone(),
                dwui_primary_300: primary.get(&300).unwrap().clone(),
                dwui_primary_400: primary.get(&400).unwrap().clone(),
                dwui_primary_500: primary.get(&500).unwrap().clone(),
                dwui_primary_600: primary.get(&600).unwrap().clone(),
                dwui_primary_700: primary.get(&700).unwrap().clone(),
                dwui_primary_800: primary.get(&800).unwrap().clone(),
                dwui_primary_900: primary.get(&900).unwrap().clone(),
                dwui_primary_950: primary.get(&950).unwrap().clone(),

                dwui_text_on_primary_50: text_on_primary.get(&50).unwrap().clone(),
                dwui_text_on_primary_100: text_on_primary.get(&100).unwrap().clone(),
                dwui_text_on_primary_200: text_on_primary.get(&200).unwrap().clone(),
                dwui_text_on_primary_300: text_on_primary.get(&300).unwrap().clone(),
                dwui_text_on_primary_400: text_on_primary.get(&400).unwrap().clone(),
                dwui_text_on_primary_500: text_on_primary.get(&500).unwrap().clone(),
                dwui_text_on_primary_600: text_on_primary.get(&600).unwrap().clone(),
                dwui_text_on_primary_700: text_on_primary.get(&700).unwrap().clone(),
                dwui_text_on_primary_800: text_on_primary.get(&800).unwrap().clone(),
                dwui_text_on_primary_900: text_on_primary.get(&900).unwrap().clone(),
                dwui_text_on_primary_950: text_on_primary.get(&950).unwrap().clone(),

                dwui_void_50: bg_void.get(&50).unwrap().clone(),
                dwui_void_100: bg_void.get(&100).unwrap().clone(),
                dwui_void_200: bg_void.get(&200).unwrap().clone(),
                dwui_void_300: bg_void.get(&300).unwrap().clone(),
                dwui_void_400: bg_void.get(&400).unwrap().clone(),
                dwui_void_500: bg_void.get(&500).unwrap().clone(),
                dwui_void_600: bg_void.get(&600).unwrap().clone(),
                dwui_void_700: bg_void.get(&700).unwrap().clone(),
                dwui_void_800: bg_void.get(&800).unwrap().clone(),
                dwui_void_900: bg_void.get(&900).unwrap().clone(),
                dwui_void_950: bg_void.get(&950).unwrap().clone(),

                // Error
                dwui_error_50: error.get(&50).unwrap().clone(),
                dwui_error_100: error.get(&100).unwrap().clone(),
                dwui_error_200: error.get(&200).unwrap().clone(),
                dwui_error_300: error.get(&300).unwrap().clone(),
                dwui_error_400: error.get(&400).unwrap().clone(),
                dwui_error_500: error.get(&500).unwrap().clone(),
                dwui_error_600: error.get(&600).unwrap().clone(),
                dwui_error_700: error.get(&700).unwrap().clone(),
                dwui_error_800: error.get(&800).unwrap().clone(),
                dwui_error_900: error.get(&900).unwrap().clone(),
                dwui_error_950: error.get(&950).unwrap().clone(),
            }
        }
    }

    fn shades(codes: [&str; 11]) -> BTreeMap<u32, String> {
        BTreeMap::from([
            (50, codes[0].to_string()),
            (100, codes[1].to_string()),
            (200, codes[2].to_string()),
            (300, codes[3].to_string()),
            (400, codes[4].to_string()),
            (500, codes[5].to_string()),
            (600, codes[6].to_string()),
            (700, codes[7].to_string()),
            (800, codes[8].to_string()),
            (900, codes[9].to_string()),
            (950, codes[10].to_string()),
        ])
    }

    /// The default success palette (dwind `apple` greens)
    pub fn default_success_palette() -> BTreeMap<u32, String> {
        use dwind::colors::codes;

        shades([
            codes::APPLE_50,
            codes::APPLE_100,
            codes::APPLE_200,
            codes::APPLE_300,
            codes::APPLE_400,
            codes::APPLE_500,
            codes::APPLE_600,
            codes::APPLE_700,
            codes::APPLE_800,
            codes::APPLE_900,
            codes::APPLE_950,
        ])
    }

    /// The default warning palette (dwind `candlelight` ambers)
    pub fn default_warning_palette() -> BTreeMap<u32, String> {
        use dwind::colors::codes;

        shades([
            codes::CANDLELIGHT_50,
            codes::CANDLELIGHT_100,
            codes::CANDLELIGHT_200,
            codes::CANDLELIGHT_300,
            codes::CANDLELIGHT_400,
            codes::CANDLELIGHT_500,
            codes::CANDLELIGHT_600,
            codes::CANDLELIGHT_700,
            codes::CANDLELIGHT_800,
            codes::CANDLELIGHT_900,
            codes::CANDLELIGHT_950,
        ])
    }

    impl Default for ColorsCssVariables {
        fn default() -> Self {
            use dwind::colors::codes;

            let bunker = shades([
                codes::BUNKER_50,
                codes::BUNKER_100,
                codes::BUNKER_200,
                codes::BUNKER_300,
                codes::BUNKER_400,
                codes::BUNKER_500,
                codes::BUNKER_600,
                codes::BUNKER_700,
                codes::BUNKER_800,
                codes::BUNKER_900,
                codes::BUNKER_950,
            ]);

            let candlelight = shades([
                codes::CANDLELIGHT_50,
                codes::CANDLELIGHT_100,
                codes::CANDLELIGHT_200,
                codes::CANDLELIGHT_300,
                codes::CANDLELIGHT_400,
                codes::CANDLELIGHT_500,
                codes::CANDLELIGHT_600,
                codes::CANDLELIGHT_700,
                codes::CANDLELIGHT_800,
                codes::CANDLELIGHT_900,
                codes::CANDLELIGHT_950,
            ]);

            let red = shades([
                codes::RED_50,
                codes::RED_100,
                codes::RED_200,
                codes::RED_300,
                codes::RED_400,
                codes::RED_500,
                codes::RED_600,
                codes::RED_700,
                codes::RED_800,
                codes::RED_900,
                codes::RED_950,
            ]);

            Self::new(&bunker, &candlelight, &bunker, &red)
        }
    }

    use dwind::border_color_generator;
    use dwind::gradient_from_generator;
    use dwind::gradient_to_generator;
    use dwind::ring_generator;
    use dwind::text_color_generator;

    dwgenerate_map!(
        "dwui-ring-primary",
        "ring-",
        [
            ("50", "var(--dwui-primary-50)"),
            ("100", "var(--dwui-primary-100)"),
            ("200", "var(--dwui-primary-200)"),
            ("300", "var(--dwui-primary-300)"),
            ("400", "var(--dwui-primary-400)"),
            ("500", "var(--dwui-primary-500)"),
            ("600", "var(--dwui-primary-600)"),
            ("700", "var(--dwui-primary-700)"),
            ("800", "var(--dwui-primary-800)"),
            ("900", "var(--dwui-primary-900)"),
            ("950", "var(--dwui-primary-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-ring-error",
        "ring-",
        [
            ("50", "var(--dwui-error-50)"),
            ("100", "var(--dwui-error-100)"),
            ("200", "var(--dwui-error-200)"),
            ("300", "var(--dwui-error-300)"),
            ("400", "var(--dwui-error-400)"),
            ("500", "var(--dwui-error-500)"),
            ("600", "var(--dwui-error-600)"),
            ("700", "var(--dwui-error-700)"),
            ("800", "var(--dwui-error-800)"),
            ("900", "var(--dwui-error-900)"),
            ("950", "var(--dwui-error-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-border-primary",
        "border-color-",
        [
            ("50", "var(--dwui-primary-50)"),
            ("100", "var(--dwui-primary-100)"),
            ("200", "var(--dwui-primary-200)"),
            ("300", "var(--dwui-primary-300)"),
            ("400", "var(--dwui-primary-400)"),
            ("500", "var(--dwui-primary-500)"),
            ("600", "var(--dwui-primary-600)"),
            ("700", "var(--dwui-primary-700)"),
            ("800", "var(--dwui-primary-800)"),
            ("900", "var(--dwui-primary-900)"),
            ("950", "var(--dwui-primary-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-border-void",
        "border-color-",
        [
            ("50", "var(--dwui-void-50)"),
            ("100", "var(--dwui-void-100)"),
            ("200", "var(--dwui-void-200)"),
            ("300", "var(--dwui-void-300)"),
            ("400", "var(--dwui-void-400)"),
            ("500", "var(--dwui-void-500)"),
            ("600", "var(--dwui-void-600)"),
            ("700", "var(--dwui-void-700)"),
            ("800", "var(--dwui-void-800)"),
            ("900", "var(--dwui-void-900)"),
            ("950", "var(--dwui-void-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-border-error",
        "border-color-",
        [
            ("50", "var(--dwui-error-50)"),
            ("100", "var(--dwui-error-100)"),
            ("200", "var(--dwui-error-200)"),
            ("300", "var(--dwui-error-300)"),
            ("400", "var(--dwui-error-400)"),
            ("500", "var(--dwui-error-500)"),
            ("600", "var(--dwui-error-600)"),
            ("700", "var(--dwui-error-700)"),
            ("800", "var(--dwui-error-800)"),
            ("900", "var(--dwui-error-900)"),
            ("950", "var(--dwui-error-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-border-success",
        "border-color-",
        [
            ("50", "var(--dwui-success-50)"),
            ("100", "var(--dwui-success-100)"),
            ("200", "var(--dwui-success-200)"),
            ("300", "var(--dwui-success-300)"),
            ("400", "var(--dwui-success-400)"),
            ("500", "var(--dwui-success-500)"),
            ("600", "var(--dwui-success-600)"),
            ("700", "var(--dwui-success-700)"),
            ("800", "var(--dwui-success-800)"),
            ("900", "var(--dwui-success-900)"),
            ("950", "var(--dwui-success-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-border-warning",
        "border-color-",
        [
            ("50", "var(--dwui-warning-50)"),
            ("100", "var(--dwui-warning-100)"),
            ("200", "var(--dwui-warning-200)"),
            ("300", "var(--dwui-warning-300)"),
            ("400", "var(--dwui-warning-400)"),
            ("500", "var(--dwui-warning-500)"),
            ("600", "var(--dwui-warning-600)"),
            ("700", "var(--dwui-warning-700)"),
            ("800", "var(--dwui-warning-800)"),
            ("900", "var(--dwui-warning-900)"),
            ("950", "var(--dwui-warning-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-text-success",
        "text-color-",
        [
            ("50", "var(--dwui-success-50)"),
            ("100", "var(--dwui-success-100)"),
            ("200", "var(--dwui-success-200)"),
            ("300", "var(--dwui-success-300)"),
            ("400", "var(--dwui-success-400)"),
            ("500", "var(--dwui-success-500)"),
            ("600", "var(--dwui-success-600)"),
            ("700", "var(--dwui-success-700)"),
            ("800", "var(--dwui-success-800)"),
            ("900", "var(--dwui-success-900)"),
            ("950", "var(--dwui-success-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-text-warning",
        "text-color-",
        [
            ("50", "var(--dwui-warning-50)"),
            ("100", "var(--dwui-warning-100)"),
            ("200", "var(--dwui-warning-200)"),
            ("300", "var(--dwui-warning-300)"),
            ("400", "var(--dwui-warning-400)"),
            ("500", "var(--dwui-warning-500)"),
            ("600", "var(--dwui-warning-600)"),
            ("700", "var(--dwui-warning-700)"),
            ("800", "var(--dwui-warning-800)"),
            ("900", "var(--dwui-warning-900)"),
            ("950", "var(--dwui-warning-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-text-primary",
        "text-color-",
        [
            ("50", "var(--dwui-primary-50)"),
            ("100", "var(--dwui-primary-100)"),
            ("200", "var(--dwui-primary-200)"),
            ("300", "var(--dwui-primary-300)"),
            ("400", "var(--dwui-primary-400)"),
            ("500", "var(--dwui-primary-500)"),
            ("600", "var(--dwui-primary-600)"),
            ("700", "var(--dwui-primary-700)"),
            ("800", "var(--dwui-primary-800)"),
            ("900", "var(--dwui-primary-900)"),
            ("950", "var(--dwui-primary-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-text-error",
        "text-color-",
        [
            ("50", "var(--dwui-error-50)"),
            ("100", "var(--dwui-error-100)"),
            ("200", "var(--dwui-error-200)"),
            ("300", "var(--dwui-error-300)"),
            ("400", "var(--dwui-error-400)"),
            ("500", "var(--dwui-error-500)"),
            ("600", "var(--dwui-error-600)"),
            ("700", "var(--dwui-error-700)"),
            ("800", "var(--dwui-error-800)"),
            ("900", "var(--dwui-error-900)"),
            ("950", "var(--dwui-error-950)")
        ]
    );

    // gradients

    dwgenerate_map!(
        "dwui-gradient-from-primary",
        "gradient-from-",
        [
            ("50", "var(--dwui-primary-50)"),
            ("100", "var(--dwui-primary-100)"),
            ("200", "var(--dwui-primary-200)"),
            ("300", "var(--dwui-primary-300)"),
            ("400", "var(--dwui-primary-400)"),
            ("500", "var(--dwui-primary-500)"),
            ("600", "var(--dwui-primary-600)"),
            ("700", "var(--dwui-primary-700)"),
            ("800", "var(--dwui-primary-800)"),
            ("900", "var(--dwui-primary-900)"),
            ("950", "var(--dwui-primary-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-gradient-to-primary",
        "gradient-to-",
        [
            ("50", "var(--dwui-primary-50)"),
            ("100", "var(--dwui-primary-100)"),
            ("200", "var(--dwui-primary-200)"),
            ("300", "var(--dwui-primary-300)"),
            ("400", "var(--dwui-primary-400)"),
            ("500", "var(--dwui-primary-500)"),
            ("600", "var(--dwui-primary-600)"),
            ("700", "var(--dwui-primary-700)"),
            ("800", "var(--dwui-primary-800)"),
            ("900", "var(--dwui-primary-900)"),
            ("950", "var(--dwui-primary-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-gradient-from-void",
        "gradient-from-",
        [
            ("50", "var(--dwui-void-50)"),
            ("100", "var(--dwui-void-100)"),
            ("200", "var(--dwui-void-200)"),
            ("300", "var(--dwui-void-300)"),
            ("400", "var(--dwui-void-400)"),
            ("500", "var(--dwui-void-500)"),
            ("600", "var(--dwui-void-600)"),
            ("700", "var(--dwui-void-700)"),
            ("800", "var(--dwui-void-800)"),
            ("900", "var(--dwui-void-900)"),
            ("950", "var(--dwui-void-950)")
        ]
    );

    dwgenerate_map!(
        "dwui-gradient-to-void",
        "gradient-to-",
        [
            ("50", "var(--dwui-void-50)"),
            ("100", "var(--dwui-void-100)"),
            ("200", "var(--dwui-void-200)"),
            ("300", "var(--dwui-void-300)"),
            ("400", "var(--dwui-void-400)"),
            ("500", "var(--dwui-void-500)"),
            ("600", "var(--dwui-void-600)"),
            ("700", "var(--dwui-void-700)"),
            ("800", "var(--dwui-void-800)"),
            ("900", "var(--dwui-void-900)"),
            ("950", "var(--dwui-void-950)")
        ]
    );
}
