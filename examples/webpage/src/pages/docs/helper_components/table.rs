use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

pub fn example_table<const COL_COUNT: usize>(
    headers: [String; COL_COUNT],
    rows: impl IntoIterator<Item = [String; COL_COUNT]>,
) -> Dom {
    html!("table", {
        .dwclass!("m-t-10 text-woodsmoke-50 divide-y border-collapse border-woodsmoke-700 w-full text-left text-sm")
        .child(html!("tr", {
            .children(headers.iter().map(|header| {
                html!("th", {
                    .dwclass!("p-b-2")
                    .text(header)
                })
            }))
        }))
        .children(rows.into_iter().map(|cells| {
            html!("tr", {
                .dwclass!("border-woodsmoke-900")
                .children(cells.into_iter().enumerate().map(|(idx, content)|{
                    if idx == 0 {
                        html!("td", {
                            .dwclass!("text-picton-blue-100 font-bold font-mono")
                            .dwclass!("p-t-2 p-b-2")
                            .text(&content)
                        })
                    } else {
                        html!("td", {
                            .dwclass!("p-t-2 p-b-2")
                            .text(&content)
                        })
                    }
                }))
            })
        }))
    })
}
