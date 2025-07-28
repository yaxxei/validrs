use std::borrow::Cow;

use crate::error::{Error, Result};

pub trait ValidateAscii {
    fn validate_ascii(&self, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Ascii);

        let Some(str) = self.str() else {
            return Err(err);
        };

        match str.is_ascii() {
            true => Ok(()),
            false => return Err(err),
        }
    }

    fn str(&self) -> Option<&str>;
}

impl ValidateAscii for String {
    fn str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateAscii for str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAscii for &str {
    fn str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateAscii for Cow<'_, str> {
    fn str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateAscii> ValidateAscii for Option<T> {
    fn str(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.str())
    }
}

impl<T: ValidateAscii> ValidateAscii for &T {
    fn str(&self) -> Option<&str> {
        (*self).str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_validation() {
        assert!("abc123".validate_ascii(None).is_ok());
        assert!("ABCXYZ".validate_ascii(None).is_ok());
        assert!("123456".validate_ascii(None).is_ok());
        assert!("abc-123".validate_ascii(None).is_ok());
        assert!("".validate_ascii(None).is_ok());
    }
}
