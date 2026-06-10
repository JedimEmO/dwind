use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

/// Derives up to two initials from a display name ("Ada Lovelace" -> "AL")
fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// A user avatar with image support and an initials fallback.
#[component(render_fn = avatar)]
struct Avatar {
    /// Image URL; when `None`, initials derived from `name` are shown
    #[signal]
    #[default(None)]
    src: Option<String>,

    /// Display name; used for the `alt` text and initials fallback
    #[signal]
    #[default("".to_string())]
    name: String,

    #[signal]
    #[default(AvatarSize::Medium)]
    size: AvatarSize,
}

pub fn avatar(props: AvatarProps) -> Dom {
    let AvatarProps {
        src,
        name,
        size,
        apply,
    } = props;

    let src = src.broadcast();
    let name = name.broadcast();
    let size = size.broadcast();

    html!("span", {
        .dwclass!("inline-flex align-items-center justify-center rounded-full overflow-hidden flex-none select-none")
        .dwclass!("dwui-bg-void-700 is(.light *):dwui-bg-void-300")
        .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-800")
        .dwclass!("border dwui-border-void-600 is(.light *):dwui-border-void-400")
        .dwclass_signal!("w-8 h-8 text-xs", size.signal().map(|v| v == AvatarSize::Small))
        .dwclass_signal!("w-10 h-10 text-sm", size.signal().map(|v| v == AvatarSize::Medium))
        .dwclass_signal!("w-14 h-14 text-base", size.signal().map(|v| v == AvatarSize::Large))
        .dwclass!("font-bold")
        .child_signal(map_ref! {
            let src = src.signal_cloned(),
            let name = name.signal_cloned() => {
                match src {
                    Some(src) => {
                        html!("img", {
                            .attr("src", src)
                            .attr("alt", name)
                            .dwclass!("w-full h-full")
                            .style("object-fit", "cover")
                        })
                    }
                    None => {
                        html!("span", {
                            .attr("aria-hidden", "true")
                            .text(&initials(name))
                        })
                    }
                }
            }
        }.map(Some))
        .attr_signal("aria-label", name.signal_cloned().map(|name| {
            if name.is_empty() {
                None
            } else {
                Some(name)
            }
        }))
        .attr("role", "img")
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

#[cfg(test)]
mod tests {
    use super::initials;

    #[test]
    fn initials_take_first_two_words() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("grace"), "G");
        assert_eq!(initials("Jean Luc Picard"), "JL");
        assert_eq!(initials(""), "");
    }
}
