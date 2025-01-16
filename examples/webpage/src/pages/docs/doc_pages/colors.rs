use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::{events, text, Dom};
use dwind::colors::DWIND_COLORS;
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use std::collections::BTreeMap;
use example_html_highlight_macro::example_html;
use crate::pages::docs::code_widget::code;

pub fn colors_page() -> Dom {
    let selected_color = Mutable::new(None);

    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Colors"))
        .text("Dwind provides a collection of preconfigured colors")
        .child(doc_page_sub_header("All the colors"))
        .child(html!("p", {
            .text(r#"DWIND includes a small selection of pre-defined colors to get you started.
They are all defined in colors.json under the resources/ directory.
DWIND uses this file to generate classes using these colors, for instanceborder-manatee-500 and bg-candlelight-950 etc.
"#)
        }))
        .child(html!("p", {
            .text(r#"It is very likely you will wish to create your own set of colors in your project.
You can do this by creating your own colors json, and add processing of it to your build.rs file.
You can see the examples directory in the DWIND repository for more information on how to make your own custom colors.
"#)
        }))

        .child(example_box(color_list(selected_color.clone()), false))
        .child_signal(selected_color.signal_ref(|selected_color| {
            selected_color.as_ref().map(|selected_color| example_box(show_selected_color(selected_color), false))
        }))

        .children(gradients())
        .children(text_color())
    })
}

pub fn show_selected_color(selected_color: &(String, u32)) -> Dom {
    let color_value = &DWIND_COLORS[&selected_color.0][&selected_color.1];

    html!("ul", {
        .dwclass!("font-mono text-woodsmoke-400")
        .child(html!("li", {
            .text(&format!("Color code: {color_value}"))
        }))
        .child(html!("li", {
            .text(&format!("bg-{}-{} ", selected_color.0, selected_color.1))
        }))
        .child(html!("li", {
            .text(&format!("text-{}-{} ", selected_color.0, selected_color.1))
        }))
        .child(html!("li", {
            .text(&format!("border-{}-{} ", selected_color.0, selected_color.1))
        }))
    })
}
pub fn color_list(selected_color: Mutable<Option<(String, u32)>>) -> Dom {
    html!("table", {
        .dwclass!("w-full text-woodsmoke-50")
        .style("min-width", "300px")
        .children(DWIND_COLORS.iter().map(|(color_name, shades)| {
            color_row(color_name, shades, selected_color.clone())
        }))
    })
}

fn color_row(
    color_name: &str,
    shades: &BTreeMap<u32, String>,
    selected_color: Mutable<Option<(String, u32)>>,
) -> Dom {
    let mut sorted = shades.iter().collect::<Vec<_>>();
    sorted.sort_by(|l, r| l.0.cmp(r.0));

    let color_name_cloned = color_name.to_string();

    html!("tr", {
        .child(html!("td", {
            .dwclass!("w-32")
            .text(&color_name)
        }))
        .child(html!("td", {
            .dwclass!("flex gap-2")
            .children(sorted.into_iter().map(clone!(color_name_cloned, selected_color => move |(shade, value)| {
                let shade = *shade;
                let is_selected_signal = selected_color.signal_cloned().map(clone!(color_name_cloned => move |selected| {
                    if let Some((selected_name, selected_shade)) = selected {
                            color_name_cloned == selected_name && shade == selected_shade
                    } else {
                        false
                    }
                }));
                html!("div", {
                   .dwclass!("rounded-sm h-10 flex-auto cursor-pointer flex align-items-center justify-center")
                   .child_signal(is_selected_signal.map(clone!(selected_color => move |selected| {
                        if selected {
                            let color = selected_color.get_cloned().unwrap();
                            let shades_map = DWIND_COLORS.get(&color.0).unwrap();
                            let mut shades = shades_map.keys().collect::<Vec<_>>();

                            shades.sort();

                            let selected_shade_index = shades.iter().position(|v| **v == color.1).unwrap();
                            let shifted_shade_position = (selected_shade_index + 6) % shades.len();
                            let shade_color = shades_map.get(shades[shifted_shade_position]).unwrap();

                            Some(html!("div", {
                                .style("background-color", format!("{shade_color}"))
                                .dwclass!("w-p-30 h-p-30")
                            }))
                        } else {
                            None
                        }
                    })))
                   .style("background-color", value)
                   .event(clone!(color_name_cloned, selected_color => move |_: events::Click| {
                       selected_color.set(Some((color_name_cloned.clone(), shade)))
                    }))
                })
            })))
        }))
    })
}

fn gradients() -> Vec<Dom> {
    vec![
        doc_page_sub_header("Gradients"),
        html!("p", {
            .text(r#"All shades of the DWIND color swatches have a corresponding
            gradient-from-[color]-[shade] and gradient-to-[color]-[shade] class generated.
            "#)
        }),
        example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                [
                    "linear-gradient-0".to_string(),
                    "Apply linear gradient, rotated 0 degrees".to_string(),
                ],
                [
                    "linear-gradient-45".to_string(),
                    "Apply linear gradient, rotated 45 degrees".to_string(),
                ],
                [
                    "linear-gradient-90".to_string(),
                    "Apply linear gradient, rotated 90 degrees".to_string(),
                ],
                [
                    "linear-gradient-135".to_string(),
                    "Apply linear gradient, rotated 135 degrees".to_string(),
                ],
                [
                    "linear-gradient-180".to_string(),
                    "Apply linear gradient, rotated 180 degrees".to_string(),
                ],
                [
                    "gradient-from-[color]-[shade]".to_string(),
                    "Apply linear gradient from color and shade".to_string(),
                ],
                [
                    "gradient-to-[color]-[shade]".to_string(),
                    "Apply linear gradient to color and shade".to_string(),
                ],
            ],
        ),

        example_box(gradient_examples(), false),
        code(&GRADIENT_EXAMPLES_EXAMPLE_HTML_MAP)
    ]
}

#[example_html(themes = ["base16-ocean.dark"])]
fn gradient_examples() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4 align-items-center w-full")
        .children([
            html!("div", {
                .dwclass!("w-p-90 h-20 rounded-md flex flex-col justify-center align-items-center")
                .dwclass!("font-mono font-extrabold")
                // Apply the gradient
                .dwclass!("linear-gradient-0 gradient-from-purple-500 gradient-to-purple-900")
                .child(text("linear-gradient-0 gradient-from-purple-500 gradient-to-purple-900"))
            }),
            html!("div", {
                .dwclass!("w-p-90 h-20 rounded-md flex flex-col justify-center align-items-center")
                .dwclass!("font-mono font-extrabold")
                // Apply the gradient
                .dwclass!("linear-gradient-45 gradient-from-purple-500 gradient-to-purple-900")
                .child(text("linear-gradient-45 gradient-from-purple-500 gradient-to-purple-900"))
            }),
            html!("div", {
                .dwclass!("w-p-90 h-20 rounded-md flex flex-col justify-center align-items-center")
                .dwclass!("font-mono font-extrabold")
                // Apply the gradient
                .dwclass!("linear-gradient-90 gradient-from-purple-500 gradient-to-purple-900")
                .child(text("linear-gradient-90 gradient-from-purple-500 gradient-to-purple-900"))
            }),
            html!("div", {
                .dwclass!("w-p-90 h-20 rounded-md flex flex-col justify-center align-items-center")
                .dwclass!("font-mono font-extrabold")
                // Apply the gradient
                .dwclass!("linear-gradient-180 gradient-from-red-500 gradient-to-picton-blue-900")
                .child(text("linear-gradient-180 gradient-from-red-500 gradient-to-picton-blue-900"))
            })
        ])
    })
}


// text color
fn text_color() -> Vec<Dom> {
    vec![
        doc_page_sub_header("Text color"),
        html!("p", {
            .text(r#"All shades of the DWIND color swatches have a corresponding
            text-[color]-[shade] class generated.
            "#)
        }),
        example_table(
            ["Class".to_string(), "Description".to_string()],
            [
                [
                    "text-[color]-[shade]".to_string(),
                    "Apply text color from color and shade".to_string(),
                ],
            ],
        ),

        example_box(text_color_example(), false),
        code(&TEXT_COLOR_EXAMPLE_EXAMPLE_HTML_MAP)
    ]
}

#[example_html(themes = ["base16-ocean.dark"])]
fn text_color_example() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4 align-items-center w-full")
        .children([
            html!("div", {
                .dwclass!("w-p-90 h-20 rounded-md flex flex-col justify-center align-items-center")
                .dwclass!("font-mono font-extrabold")
                .children([
                    html!("div", {
                        .dwclass!("text-red-300 hover:text-red-800")
                        .text("text-red-300 hover:text-red-800")
                    })
                ])
            })
        ])
    })
}

