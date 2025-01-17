use futures_signals::signal::{Signal, SignalExt};

#[derive(PartialOrd, Debug, Eq, PartialEq, Clone)]
pub enum Breakpoint {
    VerySmall,
    Small,
    Medium,
    Large,
    VeryLarge,
    MediaQuery(String),
}

impl TryFrom<&str> for Breakpoint {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "@xs" => Ok(Breakpoint::VerySmall),
            "@sm" => Ok(Breakpoint::Small),
            "@md" => Ok(Breakpoint::Medium),
            "@lg" => Ok(Breakpoint::Large),
            "@xl" => Ok(Breakpoint::VeryLarge),
            _ => Err(()),
        }
    }
}

pub fn breakpoint() -> impl Signal<Item = Breakpoint> {
    dominator::window_size()
        .map(|v| match v.width {
            v if v >= 2560.0 => Breakpoint::VeryLarge,
            v if v >= 1920.0 => Breakpoint::Large,
            v if v >= 1280.0 => Breakpoint::Medium,
            v if v >= 640.0 => Breakpoint::Small,
            _v => Breakpoint::VerySmall,
        })
        .dedupe_cloned()
}

pub fn breakpoint_active_signal(level: Breakpoint) -> impl Signal<Item = bool> {
    breakpoint().map(move |bp| bp >= level).dedupe()
}

pub fn breakpoint_less_than_signal(level: Breakpoint) -> impl Signal<Item = bool> {
    breakpoint().map(move |bp| bp < level).dedupe()
}
