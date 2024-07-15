use std::{env, fs};
use std::path::Path;

fn main() {
    let files = vec![
        "resources/css/base.css",
        "resources/css/interactivity.css",
        "resources/css/flexbox_and_grid.css",
        "resources/css/sizing.css",
        "resources/css/spacing.css",
    ];

    for file in files {
        println!("cargo:rerun-if-changed={}", file);

        let out = dominator_css_bindgen::css::generate_rust_bindings_from_file(file)
            .expect("failed to generate rust bindings");

        let out_dir = env::var_os("OUT_DIR").unwrap();
        let out_file = Path::new(Path::new(file).file_stem().unwrap()).with_extension("rs");

        let dest_path = Path::new(&out_dir).join(out_file);

        fs::write(
            &dest_path,
            out,
        ).unwrap();
    }

    println!("cargo::rerun-if-changed=build.rs");
}