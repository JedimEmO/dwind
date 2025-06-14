use dwind_build::design_tokens::render_design_token_colors_to_rust_file;
use std::path::Path;
use std::{env, fs};

fn main() {
    let out = dominator_css_bindgen::css::generate_rust_bindings_from_file("resources/simple.css")
        .expect("failed to generate rust bindings");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    fs::write(dest_path, out).unwrap();

    // Generate design token color utilities
    if let Err(e) = render_design_token_colors_to_rust_file(
        "resources/design-tokens.tokens.json",
        Path::new(&out_dir).join("design_tokens_generated.rs"),
    ) {
        eprintln!("Warning: Failed to generate design token colors: {}", e);
        eprintln!("This is expected if the design token file doesn't exist or is malformed.");
        // Create an empty file so the include! doesn't fail
        fs::write(
            Path::new(&out_dir).join("design_token_colors_generated.rs"),
            "// No design token colors generated\n",
        )
        .unwrap();
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=resources/simple.css");
    println!("cargo::rerun-if-changed=resources/design-tokens.tokens.json");
}
