use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Core,
    Navigation,
    Data,
    Charts,
    Theme,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Page::Core => write!(f, "Core"),
            Page::Navigation => write!(f, "Navigation"),
            Page::Data => write!(f, "Data Display"),
            Page::Charts => write!(f, "Charts"),
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
            Page::Charts => "#/charts",
            Page::Theme => "#/theme",
        }
    }

    pub fn all() -> Vec<Page> {
        vec![Page::Core, Page::Navigation, Page::Data, Page::Charts, Page::Theme]
    }
}

pub fn resolve_route(hash: &str) -> Page {
    let mut router = matchit::Router::new();
    router.insert("#/core", Page::Core).unwrap();
    router.insert("#/navigation", Page::Navigation).unwrap();
    router.insert("#/data", Page::Data).unwrap();
    router.insert("#/charts", Page::Charts).unwrap();
    router.insert("#/theme", Page::Theme).unwrap();

    router
        .at(hash)
        .map(|m| m.value.clone())
        .unwrap_or(Page::Core)
}
