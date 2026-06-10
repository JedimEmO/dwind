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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_result_reports_validity() {
        assert!(ValidationResult::Valid.is_valid());
        assert!(!ValidationResult::Invalid {
            message: "nope".to_string()
        }
        .is_valid());
    }

    #[test]
    fn mutable_wrapper_accepts_parsable_values() {
        let value = Mutable::new(0i32);

        let result = InputValueWrapper::set(&value, "42".to_string());

        assert!(result.is_valid());
        assert_eq!(value.get(), 42);
    }

    #[test]
    fn mutable_wrapper_rejects_unparsable_values() {
        let value = Mutable::new(7i32);

        let result = InputValueWrapper::set(&value, "bananas".to_string());

        assert!(!result.is_valid());
        // The previous value must be preserved on a failed parse
        assert_eq!(value.get(), 7);
    }

    #[test]
    fn mutable_wrapper_signal_reflects_value() {
        use futures::StreamExt;

        let value = Mutable::new(3u8);
        let mut stream = value.value_signal_cloned().to_stream();

        let current = futures::executor::block_on(stream.next()).unwrap();

        assert_eq!(current, "3");
    }

    #[test]
    fn mutable_string_wrapper_roundtrips() {
        let value = Mutable::new("".to_string());

        let result = InputValueWrapper::set(&value, "hello".to_string());

        assert!(result.is_valid());
        assert_eq!(value.get_cloned(), "hello");
    }
}
