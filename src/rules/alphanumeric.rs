use std::borrow::Cow;

use crate::error::{Error, Result};

pub trait ValidateAlphanumeric {
    fn validate_alphanumeric(&self, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Alphanumeric);

        let Some(str) = self.str() else {
            return Err(err);
        };

        match str.chars().all(|c| c.is_alphanumeric()) {
            true => Ok(()),
            false => return Err(err),
        }
    }

    fn str(&self) -> Option<&str>;
}

impl ValidateAlphanumeric for String {
    fn str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateAlphanumeric for str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAlphanumeric for &str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAlphanumeric for Cow<'_, str> {
    fn str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateAlphanumeric> ValidateAlphanumeric for Option<T> {
    fn str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.str())
    }
}

impl<T: ValidateAlphanumeric> ValidateAlphanumeric for &T {
    fn str(&self) -> Option<&str> {
        (*self).str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphanumeric_validation() {
        assert!("abc123".validate_alphanumeric(None).is_ok());
        assert!("ABCXYZ".validate_alphanumeric(None).is_ok());
        assert!("123456".validate_alphanumeric(None).is_ok());
        assert!("abc-123".validate_alphanumeric(None).is_err());
        assert!("".validate_alphanumeric(None).is_ok());
    }
}
