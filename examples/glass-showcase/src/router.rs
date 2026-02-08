use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Core,
    Navigation,
    Data,
    Theme,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Page::Core => write!(f, "Core"),
            Page::Navigation => write!(f, "Navigation"),
            Page::Data => write!(f, "Data Display"),
            Page::Theme => write!(f, "Theme"),
        }
    }
}

impl Page {
    pub fn hash(&self) -> &'static str {
        match self {
            Page::Core => "#/core",
            Page::Navigation => "#/navigation",
            Page::Data => "#/data",
            Page::Theme => "#/theme",
        }
    }

    pub fn all() -> Vec<Page> {
        vec![Page::Core, Page::Navigation, Page::Data, Page::Theme]
    }
}

pub fn resolve_route(hash: &str) -> Page {
    let mut router = matchit::Router::new();
    router.insert("#/core", Page::Core).unwrap();
    router.insert("#/navigation", Page::Navigation).unwrap();
    router.insert("#/data", Page::Data).unwrap();
    router.insert("#/theme", Page::Theme).unwrap();

    router
        .at(hash)
        .map(|m| m.value.clone())
        .unwrap_or(Page::Core)
}
