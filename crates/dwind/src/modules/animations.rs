use dwind_macros::dwkeyframes;

include!(concat!(env!("OUT_DIR"), "/animations.rs"));

// The `.animate-*` utility classes are generated from `resources/css/animations.css`
// and reference these keyframes by bare name, so the names are pinned with
// `#[name = ...]` rather than namespaced. That also keeps them stable for anyone
// who wrote the shorthand by hand.
//
// No `#[animation(...)]` here for the same reason: the classes already come from
// the CSS file, and minting a second set would shadow them.
dwkeyframes! {
    #![path = dwind_base::keyframes]
    #![register_fn = "append_animation_keyframe_style"]

    #[name = "spin"]
    spin {
        "from" => "transform: rotate(0deg);",
        "to" => "transform: rotate(360deg);",
    }

    #[name = "ping"]
    ping {
        "75%, 100%" => "transform: scale(2); opacity: 0;",
    }

    #[name = "pulse"]
    pulse {
        "0%, 100%" => "opacity: 1;",
        "50%" => "opacity: .5;",
    }

    #[name = "bounce"]
    bounce {
        "0%, 100%" => "transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1);",
        "50%" => "transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1);",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// The names are load-bearing: `resources/css/animations.css` writes
    /// `animation: spin …` by hand, so a namespaced name would silently break
    /// every `animate-*` class.
    #[test]
    fn keyframe_names_stay_unprefixed() {
        assert_eq!(SPIN_KEYFRAMES.name_unregistered(), "spin");
        assert_eq!(PING_KEYFRAMES.name_unregistered(), "ping");
        assert_eq!(PULSE_KEYFRAMES.name_unregistered(), "pulse");
        assert_eq!(BOUNCE_KEYFRAMES.name_unregistered(), "bounce");
    }

    #[test]
    fn bodies_match_the_css_they_replaced() {
        assert_eq!(
            SPIN_KEYFRAMES.css(),
            "@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"
        );
        assert_eq!(
            PULSE_KEYFRAMES.css(),
            "@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .5; } }"
        );
    }
}
