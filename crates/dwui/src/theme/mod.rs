use dominator::stylesheet;

pub fn apply_style_sheet(colors: Option<crate::theme::colors::ColorsCssVariables>) {
    stylesheet!(":root", {
        .raw(colors.unwrap_or_default().to_style_sheet_raw())
    });

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
        pub fn new(
            primary: &BTreeMap<u32, String>,
            text_on_primary: &BTreeMap<u32, String>,
            bg_void: &BTreeMap<u32, String>,
        ) -> Self {
            Self {
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
            }
        }
    }

    impl Default for ColorsCssVariables {
        fn default() -> Self {
            let primary = dwind::colors::DWIND_COLORS.get("bunker").unwrap();
            let void = dwind::colors::DWIND_COLORS.get("bunker").unwrap();
            let text_on_primary = dwind::colors::DWIND_COLORS.get("candlelight").unwrap();

            Self::new(primary, text_on_primary, void)
        }
    }
}
