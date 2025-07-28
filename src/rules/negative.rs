use std::fmt::Debug;

use crate::error::{Error, Result};

pub trait ValidateNegative
where
    Self: PartialOrd + Debug + Copy,
{
    fn validate_negative(&self, msg: Option<String>) -> Result<()> {
        if *self >= Self::zero() {
            return Err(msg.map(Error::Custom).unwrap_or(Error::Negative));
        }
        Ok(())
    }

    fn zero() -> Self;
}

macro_rules! validate_numbers {
    ($type:ty) => {
        impl ValidateNegative for $type {
            fn zero() -> Self {
                match stringify!($type) {
                    "f32" | "f64" => 0.0 as $type,
                    _ => 0 as $type,
                }
            }
        }
    };
}

validate_numbers!(i8);
validate_numbers!(u8);
validate_numbers!(i16);
validate_numbers!(u16);
validate_numbers!(i32);
validate_numbers!(u32);
validate_numbers!(i64);
validate_numbers!(u64);
validate_numbers!(i128);
validate_numbers!(u128);
validate_numbers!(usize);
validate_numbers!(f32);
validate_numbers!(f64);

#[cfg(test)]
mod test {
    use super::ValidateNegative;

    #[test]
    fn test_validate_number() {
        assert!((-3).validate_negative(None).is_ok());
    }
}
