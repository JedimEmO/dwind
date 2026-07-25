//! Every animation on the site, declared in Rust.
//!
//! These used to be a 90-line `@keyframes` blob in a raw stylesheet. `dwkeyframes!`
//! emits the at-rule *and* — where `#[animation(...)]` is given — a
//! compile-time-checked `animate-*` class, and only injects a rule if something
//! actually uses it.
//!
//! Where an animation needs a delay computed at runtime, use the handle
//! directly: `format!("{FADE_UP_KEYFRAMES} 600ms {}ms ease-out both", i * 60)`.
//! Formatting the handle registers the rule, so the shorthand can never point at
//! a keyframe that was never injected.

use dwind_macros::dwkeyframes;

dwkeyframes! {
    #![prefix = "dw"]

    /// Terminal cursor blink on the wordmark.
    #[animation("1.2s step-end infinite")]
    cursor_blink {
        "0%, 49%" => "opacity: 1;",
        "50%, 100%" => "opacity: 0;",
    }

    /// The workhorse entrance. Used with computed delays, so no class.
    fade_up {
        "from" => "opacity: 0; transform: translateY(14px);",
        "to" => "opacity: 1; transform: translateY(0);",
    }

    /// Slow drift for the background colour field.
    aurora_a {
        "0%, 100%" => "transform: translate3d(0, 0, 0) scale(1);",
        "33%" => "transform: translate3d(6%, -8%, 0) scale(1.15);",
        "66%" => "transform: translate3d(-5%, 5%, 0) scale(0.95);",
    }

    aurora_b {
        "0%, 100%" => "transform: translate3d(0, 0, 0) scale(1.05);",
        "50%" => "transform: translate3d(-8%, 6%, 0) scale(0.9);",
    }

    /// Sweeps the gold sheen across a gradient headline.
    #[animation("7s linear infinite")]
    sheen {
        "0%" => "background-position: 0% 50%;",
        "100%" => "background-position: 200% 50%;",
    }

    /// Headline words ride in one at a time; the delay is per-word.
    word_in {
        "from" => "opacity: 0; transform: translateY(0.7em) rotate(2deg);",
        "to" => "opacity: 1; transform: translateY(0) rotate(0deg);",
    }

    /// The utility-class ticker. The track is rendered twice, so -50% loops.
    #[animation("42s linear infinite")]
    marquee {
        "from" => "transform: translate3d(0, 0, 0);",
        "to" => "transform: translate3d(-50%, 0, 0);",
    }

    #[animation("240ms cubic-bezier(0.16, 1, 0.3, 1) both")]
    palette_in {
        "from" => "opacity: 0; transform: translateY(-12px) scale(0.98);",
        "to" => "opacity: 1; transform: translateY(0) scale(1);",
    }

    #[animation("180ms ease-out both")]
    scrim_in {
        "from" => "opacity: 0;",
        "to" => "opacity: 1;",
    }

    #[animation("420ms cubic-bezier(0.16, 1, 0.3, 1) both")]
    route_in {
        "from" => "opacity: 0; transform: translateY(10px);",
        "to" => "opacity: 1; transform: translateY(0);",
    }

    /// Expanding ring behind a status dot.
    #[animation("2.2s ease-out infinite")]
    pulse_ring {
        "0%" => "box-shadow: 0 0 0 0 rgba(213, 182, 95, 0.35);",
        "70%" => "box-shadow: 0 0 0 12px rgba(213, 182, 95, 0);",
        "100%" => "box-shadow: 0 0 0 0 rgba(213, 182, 95, 0);",
    }
}
