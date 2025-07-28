// use crate::error::Result;

use std::borrow::Cow;

use crate::rules::AsStr;

#[derive(Debug, thiserror::Error, Clone)]
pub enum PhoneError {
    #[error("Invalid phone number")]
    Invalid,
    #[error("{0}")]
    Custom(String),
}

pub trait ValidatePhone: AsStr {
    fn validate_phone(&self, msg: Option<String>) -> Result<(), PhoneError> {
        let err = msg.map(PhoneError::Custom).unwrap_or(PhoneError::Invalid);
        let phone_str = self.as_str();

        phonenumber::parse(None, phone_str).map_err(|_| err)?;
        Ok(())
    }
}

impl<T: AsStr> ValidatePhone for T {}

#[cfg(test)]
mod tests {
    use super::ValidatePhone;

    #[test]
    fn test_valid_international_numbers() {
        assert!("+12125551212".validate_phone(None).is_ok()); // США
        assert!("+442072222222".validate_phone(None).is_ok()); // UK
        assert!("+74951234567".validate_phone(None).is_ok()); // Россия
        assert!(Some("+74951234567").validate_phone(None).is_ok()); // Россия
    }

    #[test]
    fn test_invalid_numbers() {
        assert!("12345".validate_phone(None).is_err()); // Слишком короткий
        assert!("+9991234567".validate_phone(None).is_err()); // Несуществующий код
        assert!("abcdefg".validate_phone(None).is_err()); // Не цифры
    }
}
