use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub name: String,
    pub shades: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize)]
pub struct ColorFile {
    pub colors: Vec<Color>,
}

pub fn render_color_json_file_to_rust_file(
    color_file_path: impl AsRef<Path>,
    colors_out_file: impl AsRef<Path>,
) {
    // Generate the colors file
    let color_file = fs::read_to_string(color_file_path).unwrap();
    let colors: ColorFile = serde_json::from_str(&color_file).unwrap();

    let mut colors_out_file = fs::File::create(colors_out_file).unwrap();

    for color in colors.colors.iter() {
        let _ = colors_out_file
            .write(render_color(color, "bg", "bg-color-").as_bytes())
            .unwrap();
        let _ = colors_out_file
            .write(render_color(color, "text", "text-color-").as_bytes())
            .unwrap();
        let _ = colors_out_file
            .write(render_color(color, "border", "border-color-").as_bytes())
            .unwrap();

        let _ = colors_out_file
            .write(render_color(color, "gradient-from", "gradient-from-").as_bytes())
            .unwrap();
        let _ = colors_out_file
            .write(render_color(color, "gradient-to", "gradient-to-").as_bytes())
            .unwrap();
        let _ = colors_out_file
            .write(render_color(color, "from", "gradient-from-").as_bytes())
            .unwrap();

        let _ = colors_out_file
            .write(render_color(color, "to", "gradient-to-").as_bytes())
            .unwrap();

        let _ = colors_out_file
            .write(render_color(color, "fill", "fill-").as_bytes())
            .unwrap();

        let _ = colors_out_file
            .write(render_color(color, "stroke", "stroke-").as_bytes())
            .unwrap();

        let _ = colors_out_file
            .write(render_color(color, "ring", "ring-").as_bytes())
            .unwrap();
    }

    let _ = colors_out_file
        .write(render_dwind_colors(&colors).as_bytes())
        .unwrap();
}

// render a once cell with a hash map of all the colors
fn render_dwind_colors(colors: &ColorFile) -> String {
    let colors = colors
        .colors
        .iter()
        .map(|color| {
            let shades = color
                .shades
                .iter()
                .map(|(shade, value)| format!("({shade}, \"{value}\".to_string())"))
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "(\"{}\".to_string(), std::collections::BTreeMap::from([{shades}]))",
                color.name
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"pub static DWIND_COLORS: once_cell::sync::Lazy<std::collections::BTreeMap<String, std::collections::BTreeMap<u32, String>>> = once_cell::sync::Lazy::new(|| {{
    std::collections::BTreeMap::from([
    {colors}
])
}});"#
    )
}

fn render_color(color: &Color, prefix: &str, generator: &str) -> String {
    let shades = color
        .shades
        .iter()
        .map(|(intensity, value)| format!("    (\"{}-{intensity}\", \"{value}\")", color.name))
        .collect::<Vec<String>>()
        .join(",\n");

    format!(
        r#"dwgenerate_map!("{prefix}", "{generator}", [
{shades}
]);"#,
    )
}
