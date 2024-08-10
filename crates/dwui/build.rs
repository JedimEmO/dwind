use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

#[derive(Serialize, Deserialize)]
struct Color {
    name: String,
    shades: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize)]
struct ColorFile {
    colors: Vec<Color>,
}

fn main() {
    let files = vec![
        "resources/css/base.css",
        "resources/css/colors.css"
    ];

    let out_dir = env::var_os("OUT_DIR").unwrap();

    for file in files {
        println!("cargo:rerun-if-changed={}", file);

        let out = dominator_css_bindgen::css::generate_rust_bindings_from_file(file)
            .expect("failed to generate rust bindings");

        let out_file = Path::new(Path::new(file).file_stem().unwrap()).with_extension("rs");

        let dest_path = Path::new(&out_dir).join(out_file);

        fs::write(&dest_path, out).unwrap();
    }

    println!("cargo::rerun-if-changed=build.rs");
}
