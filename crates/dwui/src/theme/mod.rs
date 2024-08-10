use dominator::stylesheet;

pub mod classes;

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
    include!(concat!(env!("OUT_DIR"), "/colors.rs"));

    impl Default for ColorsCssVariables {
        fn default() -> Self {
            Self {
                dwui_light_primary_50: "#fbf4f7".to_string(),
                dwui_light_primary_100: "#f8ebf0".to_string(),
                dwui_light_primary_200: "#f2d8e4".to_string(),
                dwui_light_primary_300: "#e9b8cd".to_string(),
                dwui_light_primary_400: "#d57b9f".to_string(),
                dwui_light_primary_500: "#cc688d".to_string(),
                dwui_light_primary_600: "#b74b6e".to_string(),
                dwui_light_primary_700: "#9d3957".to_string(),
                dwui_light_primary_800: "#833148".to_string(),
                dwui_light_primary_900: "#6e2d3f".to_string(),
                dwui_light_primary_950: "#421521".to_string(),
                dwui_light_text_on_primary_50: "#f3f7f8".to_string(),
                dwui_light_text_on_primary_100: "#e0e9ed".to_string(),
                dwui_light_text_on_primary_200: "#c4d5dd".to_string(),
                dwui_light_text_on_primary_300: "#9bb7c5".to_string(),
                dwui_light_text_on_primary_400: "#6b90a5".to_string(),
                dwui_light_text_on_primary_500: "#50758a".to_string(),
                dwui_light_text_on_primary_600: "#456275".to_string(),
                dwui_light_text_on_primary_700: "#3d5161".to_string(),
                dwui_light_text_on_primary_800: "#374653".to_string(),
                dwui_light_text_on_primary_900: "#323c47".to_string(),
                dwui_light_text_on_primary_950: "#0e1216".to_string(),
            }
        }
    }
}
