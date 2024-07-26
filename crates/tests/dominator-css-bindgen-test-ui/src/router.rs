use matchit::{Params, Router};
use wasm_bindgen::UnwrapThrowExt;
use dominator::routing::url;
use futures_signals::signal::Signal;
use futures_signals::signal::SignalExt;

pub struct AppRouter<TValue> {
    router: Router<Box<dyn Fn(Params) -> Result<TValue, ()>>>,
}

impl<TValue> AppRouter<TValue> where TValue: Default {
    pub fn new(router: Router<Box<dyn Fn(Params) -> Result<TValue, ()>>>) -> Self {
        Self { router }
    }

    #[inline]
    fn match_url(&self, url: impl AsRef<str>) -> Result<TValue, ()> {
        let matched = self.router.at(url.as_ref()).map_err(|_| ())?;

        (matched.value)(matched.params)
    }

    pub fn signal(self) -> impl Signal<Item=TValue> {
        url().signal_ref(|current_route| {
            web_sys::Url::new(current_route.as_str()).expect_throw("Invalid url")
        }).map(move |new_url| {
            if let Ok(route) = self.match_url(new_url.hash().as_str()) {
                info!("url: {}", new_url.hash().as_str());
                route
            } else {
                info!("unmatched url: {}", new_url.hash().as_str());
                TValue::default()
            }
        })
    }
}