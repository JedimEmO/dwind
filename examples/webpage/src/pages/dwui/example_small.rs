use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;

pub fn dwui_example_small() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(html!("div", {
            .dwclass!("flex justify-center align-items-center gap-4 @<sm:flex-col @sm:flex-row")
            .child(example_card(ColorScheme::Primary))
            .child(example_card(ColorScheme::Void))
        }))
    })
}

fn example_card(scheme: ColorScheme) -> Dom {
    card!({
        .scheme(scheme)
        .apply(|b| {
            dwclass!(b, "w-64 h-64 flex-initial")
        })
    })
}