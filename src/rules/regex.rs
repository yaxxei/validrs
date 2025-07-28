use regex::Regex;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum RegexError {
    #[error("String doesn't match the pattern")]
    NoMatch,

    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(String),

    #[error("{0}")]
    Custom(String),
}

pub type RegexResult<T> = Result<T, RegexError>;

pub trait ValidateRegex {
    fn validate_regex(&self, regex: &Regex, msg: Option<String>) -> RegexResult<()> {
        let err = msg.map(RegexError::Custom).unwrap_or(RegexError::NoMatch);
        let s = self.regex_str().ok_or(err.clone())?;
        if !regex.is_match(s) {
            return Err(err);
        }
        Ok(())
    }

    fn validate_regex_pattern(&self, pattern: &str, msg: Option<String>) -> RegexResult<()> {
        let regex = Regex::new(pattern).map_err(|e| RegexError::InvalidPattern(e.to_string()))?;
        self.validate_regex(&regex, msg)
    }

    fn regex_str(&self) -> Option<&str>;
}

impl ValidateRegex for String {
    fn regex_str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateRegex for str {
    fn regex_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateRegex for &str {
    fn regex_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateRegex for Cow<'_, str> {
    fn regex_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateRegex> ValidateRegex for Option<T> {
    fn regex_str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.regex_str())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static TEST_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\d{3}-\d{2}-\d{4}$").unwrap());

    static EMAIL_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

    static PHONE_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\+?[\d\s-]{10,}$").unwrap());

    #[test]
    fn test_regex_validation() {
        // Проверка с предварительно скомпилированным regex
        assert!("123-45-6789".validate_regex(&TEST_REGEX, None).is_ok());
        assert!("invalid".validate_regex(&TEST_REGEX, None).is_err());

        // Проверка с строковым шаблоном
        assert!(
            "123-45-6789"
                .validate_regex_pattern(r"\d{3}-\d{2}-\d{4}", None)
                .is_ok()
        );
        assert!("invalid".validate_regex_pattern(r"\d+", None).is_err());

        // Проверка кастомного сообщения
        let err = "invalid"
            .validate_regex_pattern(r"\d+", Some("Custom error".into()))
            .unwrap_err();
        assert!(matches!(err, RegexError::Custom(_)));

        // Проверка с lazy_static regex
        assert!(
            "test@example.com"
                .validate_regex(&EMAIL_REGEX, None)
                .is_ok()
        );
        assert!("+1234567890".validate_regex(&PHONE_REGEX, None).is_ok());
    }

    #[test]
    fn test_invalid_pattern() {
        let result = "test".validate_regex_pattern(r"invalid[", None);
        assert!(matches!(result, Err(RegexError::InvalidPattern(_))));
    }

    #[test]
    fn test_different_types() {
        assert!(
            String::from("123-45-6789")
                .validate_regex(&TEST_REGEX, None)
                .is_ok()
        );
        assert!(
            Some("123-45-6789")
                .validate_regex(&TEST_REGEX, None)
                .is_ok()
        );
        assert!(None::<&str>.validate_regex(&TEST_REGEX, None).is_err());
        assert!(
            Cow::Borrowed("123-45-6789")
                .validate_regex(&TEST_REGEX, None)
                .is_ok()
        );
    }
}
