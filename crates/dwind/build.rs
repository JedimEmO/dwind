use std::path::Path;
use std::{env, fs};
use std::collections::HashMap;
use std::io::Write;
use serde::{Deserialize, Serialize};

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
        "resources/css/borders.css",
        "resources/css/box_shadow.css",
        "resources/css/interactivity.css",
        "resources/css/flexbox_and_grid.css",
        "resources/css/position.css",
        "resources/css/sizing.css",
        "resources/css/spacing.css",
        "resources/css/typography.css",
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

    // Generate the colors file
    let color_file = fs::read_to_string("resources/colors.json").unwrap();
    let colors: ColorFile = serde_json::from_str(&color_file).unwrap();

    let colors_out_file = Path::new(&out_dir).join("colors_generators.rs");
    let mut colors_out_file = fs::File::create(colors_out_file).unwrap();

    for color in colors.colors.iter() {
        colors_out_file.write(render_color(color, "bg", "bg-color-").as_bytes()).unwrap();
        colors_out_file.write(render_color(color, "text", "text-color-").as_bytes()).unwrap();
        colors_out_file.write(render_color(color, "border", "border-color-").as_bytes()).unwrap();
    }

    colors_out_file.write(render_dwind_colors(&colors).as_bytes()).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=resources/colors.json");
}

// render a once cell with a hash map of all the colors
fn render_dwind_colors(colors: &ColorFile) -> String {
    let colors = colors.colors.iter().map(|color| {
        let shades = color.shades.iter().map(|(shade, value)| {
            format!("({shade}, \"{value}\".to_string())")
        }).collect::<Vec<_>>().join(", ");

        format!("(\"{}\".to_string(), std::collections::BTreeMap::from([{shades}]))", color.name)
    }).collect::<Vec<_>>().join(",\n");

    format!(r#"pub static DWIND_COLORS: once_cell::sync::Lazy<std::collections::BTreeMap<String, std::collections::BTreeMap<u32, String>>> = once_cell::sync::Lazy::new(|| {{
    std::collections::BTreeMap::from([
    {colors}
])
}});"#)
}
fn render_color(color: &Color, prefix: &str, generator: &str) -> String {
    let shades = color.shades.iter().map(|(intensity, value)| {
        format!("    (\"{}-{intensity}\", \"{value}\")", color.name)
    }).collect::<Vec<String>>().join(",\n");

    format!(r#"dwgenerate_map!("{prefix}", "{generator}", [
{shades}
]);"#, )
}