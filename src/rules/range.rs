use std::{
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use crate::error::Result;

pub trait ValidateRange {
    fn validate_range(
        &self,
        min: Option<usize>,
        max: Option<usize>,
        msg: Option<String>,
    ) -> Result<()> {
        match (min, max, msg) {
            (Some(_), Some(_), Some(msg)) => {
                if self.value() < min || self.value() > max {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (Some(_), Some(_), None) => {
                if self.value() < min || self.value() > max {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (Some(_), None, Some(msg)) => {
                if self.value() < min {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (Some(_), None, None) => {
                if self.value() < min {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (None, Some(_), Some(msg)) => {
                if self.value() > max {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (None, Some(_), None) => {
                if self.value() > max {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (None, None, None) | (None, None, Some(_)) => (),
        }

        Ok(())
    }

    fn value(&self) -> Option<usize>;
}

macro_rules! validate_num {
    ($type:ty) => {
        impl ValidateRange for $type {
            fn value(&self) -> Option<usize> {
                Some(*self as usize)
            }
        }
    };
}

validate_num!(i8);
validate_num!(u8);
validate_num!(i16);
validate_num!(u16);
validate_num!(i32);
validate_num!(u32);
validate_num!(i64);
validate_num!(u64);
validate_num!(i128);
validate_num!(u128);
validate_num!(usize);

macro_rules! validate_type_with_deref {
    ($type:ty) => {
        impl<T: ValidateRange> ValidateRange for $type {
            fn value(&self) -> Option<usize> {
                T::value(self)
            }
        }
    };
}

validate_type_with_deref!(Box<T>);
validate_type_with_deref!(Arc<T>);
validate_type_with_deref!(Rc<T>);
validate_type_with_deref!(Ref<'_, T>);
validate_type_with_deref!(RefMut<'_, T>);

impl<T: ValidateRange> ValidateRange for Option<T> {
    fn value(&self) -> Option<usize> {
        let Some(s) = self else {
            return None;
        };

        T::value(s)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidateRange;

    #[test]
    fn test_validate_length() {
        let valid = 5.validate_range(Some(1), Some(10), None).unwrap();
        assert_eq!(valid, ());
    }
}
