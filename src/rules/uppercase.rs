use std::borrow::Cow;

use crate::error::{Error, Result};

pub trait ValidateUppercase {
    fn validate_uppercase(&self, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Lowercase);

        let Some(str) = self.str() else {
            return Err(err);
        };

        match str.chars().all(|c| c.is_uppercase()) {
            true => Ok(()),
            false => return Err(err),
        }
    }

    fn str(&self) -> Option<&str>;
}

impl ValidateUppercase for String {
    fn str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateUppercase for str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateUppercase for &str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateUppercase for Cow<'_, str> {
    fn str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateUppercase> ValidateUppercase for Option<T> {
    fn str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.str())
    }
}

impl<T: ValidateUppercase> ValidateUppercase for &T {
    fn str(&self) -> Option<&str> {
        (*self).str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_validation() {
        assert!("ABCXYZ".validate_uppercase(None).is_ok());
        assert!("abcdef".validate_uppercase(None).is_err());
        assert!("aBc".validate_uppercase(None).is_err());
        assert!("".validate_uppercase(None).is_ok());
    }
}
