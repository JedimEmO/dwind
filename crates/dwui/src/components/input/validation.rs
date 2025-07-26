use futures_signals::signal::{LocalBoxSignal, Mutable, SignalExt};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid { message: String },
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}

pub trait InputValueWrapper {
    fn set(&self, value: String) -> ValidationResult;
    fn value_signal_cloned(&self) -> LocalBoxSignal<'static, String>;
}

impl<T> InputValueWrapper for Mutable<T>
where
    T: Clone + ToString + FromStr + 'static,
{
    fn set(&self, value: String) -> ValidationResult {
        if let Ok(v) = T::from_str(&value) {
            self.set(v);

            ValidationResult::Valid
        } else {
            ValidationResult::Invalid {
                message: "Invalid value".to_string(),
            }
        }
    }

    fn value_signal_cloned(&self) -> LocalBoxSignal<'static, String> {
        self.signal_cloned().map(|v| v.to_string()).boxed_local()
    }
}
