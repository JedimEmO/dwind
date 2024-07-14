use std::{env, fs};
use std::path::Path;

fn main() {
    let out = dominator_css_bindgen::css::generate_rust_bindings_from_file("resources/css/base.css")
        .expect("failed to generate rust bindings");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("base.rs");

    fs::write(
        &dest_path,
        out,
    ).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=resources/css/base.css");
}