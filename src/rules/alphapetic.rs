use std::borrow::Cow;

use crate::error::{Error, Result};

pub trait ValidateAlphabetic {
    fn validate_alphabetic(&self, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Alphabetic);

        let Some(str) = self.str() else {
            return Err(err);
        };

        match str.chars().all(|c| c.is_alphabetic()) {
            true => Ok(()),
            false => return Err(err),
        }
    }

    fn str(&self) -> Option<&str>;
}

impl ValidateAlphabetic for String {
    fn str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateAlphabetic for str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAlphabetic for &str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAlphabetic for Cow<'_, str> {
    fn str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateAlphabetic> ValidateAlphabetic for Option<T> {
    fn str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.str())
    }
}

impl<T: ValidateAlphabetic> ValidateAlphabetic for &T {
    fn str(&self) -> Option<&str> {
        (*self).str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabetic_validation() {
        assert!("abcdef".validate_alphabetic(None).is_ok());
        assert!("ABCXYZ".validate_alphabetic(None).is_ok());
        assert!("123456".validate_alphabetic(None).is_err());
        assert!("abc123".validate_alphabetic(None).is_err());
        assert!("".validate_alphabetic(None).is_ok());
    }
}
