// Re-export validation types from dwui to avoid duplication
pub use dwui::prelude::{InputValueWrapper, ValidationResult};

use futures_signals::signal::{LocalBoxSignal, Mutable, SignalExt};

// -- Select / Radio shared types --

#[derive(Debug, Clone, PartialEq)]
pub struct GlassSelectOption {
    pub label: String,
    pub value: String,
}

pub trait GlassSelectValue {
    fn get_signal(&self) -> LocalBoxSignal<'static, Option<String>>;
    fn set(&self, value: Option<String>);
}

impl GlassSelectValue for Mutable<Option<String>> {
    fn get_signal(&self) -> LocalBoxSignal<'static, Option<String>> {
        self.signal_cloned().boxed_local()
    }

    fn set(&self, value: Option<String>) {
        Mutable::set(self, value);
    }
}

impl<T: GlassSelectValue + ?Sized> GlassSelectValue for Box<T> {
    fn get_signal(&self) -> LocalBoxSignal<'static, Option<String>> {
        (**self).get_signal()
    }

    fn set(&self, value: Option<String>) {
        (**self).set(value)
    }
}

// -- Toggle / Checkbox shared types --

pub trait GlassToggleValue {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool>;
    fn toggle(&self);
}

impl GlassToggleValue for Mutable<bool> {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool> {
        self.signal().boxed_local()
    }

    fn toggle(&self) {
        self.set(!self.get());
    }
}

impl<T: GlassToggleValue + ?Sized> GlassToggleValue for Box<T> {
    fn get_signal(&self) -> LocalBoxSignal<'static, bool> {
        (**self).get_signal()
    }

    fn toggle(&self) {
        (**self).toggle()
    }
}
