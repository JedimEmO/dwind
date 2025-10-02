use dominator::html;

include!(concat!(env!("OUT_DIR"), "/animations.rs"));

const ANIMATION_KEYFRAMES: &str = r#"
@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

@keyframes ping {
    75%, 100% {
        transform: scale(2);
        opacity: 0;
    }
}

@keyframes pulse {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: .5;
    }
}

@keyframes bounce {
    0%, 100% {
        transform: translateY(-25%);
        animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
    }
    50% {
        transform: translateY(0);
        animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
    }
}"#;

pub fn append_animation_keyframe_style() {
    dominator::stylesheet_raw(ANIMATION_KEYFRAMES);
}
