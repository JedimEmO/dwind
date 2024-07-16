use dominator::{Dom, html};
use futures_signals::signal::option;
use futures_signals_component_macro::component;
use dwind::sizing::*;
use dwind::typography::*;

#[component(render_fn = heading)]
struct Heading {
    #[signal]
    content: Dom,
}

pub fn heading(props: impl HeadingPropsTrait + 'static) -> Dom {
    let HeadingProps { content, apply } = props.take();

    html!("div", {
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .dwclass!("w-auto font-semibold")
        .child(html!("h1", {
            .dwclass!("w-auto font-semibold text-xl")
            .child_signal(option(content))
        }))
    })
}