use dominator::stylesheet;

pub mod classes;

pub fn apply_style_sheet() {
    stylesheet!(":root", {
        .raw(r###"--dwui-light-primary-950: #421521;
--dwui-light-primary-900: #6e2d3f;
--dwui-light-primary-800: #833148;
--dwui-light-primary-700: #9d3957;
--dwui-light-primary-600: #b74b6e;
--dwui-light-primary-500: #cc688d;
--dwui-light-primary-400: #d57b9f;
--dwui-light-primary-300: #e9b8cd;
--dwui-light-primary-200: #f2d8e4;
--dwui-light-primary-100: #f8ebf0;
--dwui-light-primary-50: #fbf4f7;
--dwui-light-text-on-primary-950: #0e1216;
--dwui-light-text-on-primary-900: #323c47;
--dwui-light-text-on-primary-800: #374653;
--dwui-light-text-on-primary-700: #3d5161;
--dwui-light-text-on-primary-600: #456275;
--dwui-light-text-on-primary-500: #50758a;
--dwui-light-text-on-primary-400: #6b90a5;
--dwui-light-text-on-primary-300: #9bb7c5;
--dwui-light-text-on-primary-200: #c4d5dd;
--dwui-light-text-on-primary-100: #e0e9ed;
--dwui-light-text-on-primary-50: #f3f7f8;
"###)
    });

    base::apply_base_stylesheet();
    button::apply_button_stylesheet();
}

pub mod base {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub mod button {
    include!(concat!(env!("OUT_DIR"), "/button.rs"));
}
