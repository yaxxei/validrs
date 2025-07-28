use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::error::{Error, Result};

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap()
});

pub trait ValidateEmail {
    fn validate_email(&self, msg: Option<String>) -> Result<()> {
        if let Some(email) = self.email_string() {
            if !EMAIL_REGEX.is_match(&email) {
                return Err(msg.map(Error::Custom).unwrap_or(Error::Email));
            }
        }
        Ok(())
    }

    fn email_string(&self) -> Option<Cow<str>>;
}

impl ValidateEmail for String {
    fn email_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

impl ValidateEmail for &str {
    fn email_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(*self))
    }
}

impl ValidateEmail for Cow<'_, str> {
    fn email_string(&self) -> Option<Cow<str>> {
        Some(self.clone())
    }
}

impl<T> ValidateEmail for &T
where
    T: ValidateEmail,
{
    fn email_string(&self) -> Option<Cow<str>> {
        T::email_string(self)
    }
}

impl<T: ValidateEmail> ValidateEmail for Option<T> {
    fn email_string(&self) -> Option<Cow<str>> {
        let Some(s) = self else {
            return None;
        };
        T::email_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidateEmail;

    #[test]
    fn test_validate_email() {
        assert!("test@gmail.com".validate_email(None).is_ok());
    }
}
