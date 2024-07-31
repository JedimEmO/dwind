use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::{events, Dom};
use dwind::colors::DWIND_COLORS;
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use std::collections::BTreeMap;

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
        .dwclass!("w-full")
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
                            let shifted_shade_position = (selected_shade_index  - 6) % shades.len();
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
