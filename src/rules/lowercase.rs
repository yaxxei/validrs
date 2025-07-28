use std::borrow::Cow;

use crate::error::{Error, Result};

pub trait ValidateLowercase {
    fn validate_lowercase(&self, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Lowercase);

        let Some(str) = self.str() else {
            return Err(err);
        };

        match str.chars().all(|c| c.is_lowercase()) {
            true => Ok(()),
            false => return Err(err),
        }
    }

    fn str(&self) -> Option<&str>;
}

impl ValidateLowercase for String {
    fn str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateLowercase for str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateLowercase for &str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateLowercase for Cow<'_, str> {
    fn str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateLowercase> ValidateLowercase for Option<T> {
    fn str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.str())
    }
}

impl<T: ValidateLowercase> ValidateLowercase for &T {
    fn str(&self) -> Option<&str> {
        (*self).str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_validation() {
        assert!("abcdef".validate_lowercase(None).is_ok());
        assert!("ABCXYZ".validate_lowercase(None).is_err());
        assert!("aBc".validate_lowercase(None).is_err());
        assert!("".validate_lowercase(None).is_ok());
    }
}
