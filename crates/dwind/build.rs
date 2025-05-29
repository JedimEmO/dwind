use dwind_build::colors::render_color_json_file_to_rust_file;
use dwind_build::design_tokens::render_design_token_colors_to_rust_file;
use std::path::Path;
use std::{env, fs};

fn main() {
    let files = vec![
        "resources/css/base.css",
        "resources/css/borders.css",
        "resources/css/box_shadow.css",
        "resources/css/colors.css",
        "resources/css/effects.css",
        "resources/css/interactivity.css",
        "resources/css/filters.css",
        "resources/css/flexbox_and_grid.css",
        "resources/css/layout.css",
        "resources/css/position.css",
        "resources/css/sizing.css",
        "resources/css/spacing.css",
        "resources/css/typography.css",
        "resources/css/transforms.css",
        "resources/css/transition.css",
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

    render_color_json_file_to_rust_file(
        "resources/colors.json",
        Path::new(&out_dir).join("colors_generated.rs"),
    );

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
        ).unwrap();
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=resources/colors.json");
    println!("cargo::rerun-if-changed=resources/design-tokens-extended.tokens.json");
}
