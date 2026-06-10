use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

pub fn example_table<const COL_COUNT: usize>(
    headers: [String; COL_COUNT],
    rows: impl IntoIterator<Item = [String; COL_COUNT]>,
) -> Dom {
    html!("div", {
        .dwclass!("m-t-6 rounded-lg border border-woodsmoke-800 overflow-x-auto")
        .style("background", "rgba(18, 18, 21, 0.5)")
        .child(html!("table", {
            .dwclass!("text-woodsmoke-100 border-collapse w-full text-left text-sm")
            .child(html!("tr", {
                .dwclass!("border-b border-woodsmoke-800")
                .children(headers.iter().map(|header| {
                    html!("th", {
                        .class("font-code")
                        .dwclass!("p-3 text-xs text-woodsmoke-500 font-medium")
                        .text(&format!("// {}", header.to_lowercase()))
                    })
                }))
            }))
            .children(rows.into_iter().map(|cells| {
                html!("tr", {
                    .dwclass!("border-b border-woodsmoke-900")
                    .children(cells.into_iter().enumerate().map(|(idx, content)|{
                        if idx == 0 {
                            html!("td", {
                                .class("font-code")
                                .dwclass!("text-candlelight-300 p-3")
                                .text(&content)
                            })
                        } else {
                            html!("td", {
                                .dwclass!("p-3 text-woodsmoke-300")
                                .text(&content)
                            })
                        }
                    }))
                })
            }))
        }))
    })
}
