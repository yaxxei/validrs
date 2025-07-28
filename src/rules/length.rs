use std::{
    cell::{Ref, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

use crate::error::Result;

pub trait ValidateLength {
    fn validate_length(
        &self,
        min: Option<usize>,
        max: Option<usize>,
        msg: Option<String>,
    ) -> Result<()> {
        match (min, max, msg) {
            (Some(_), Some(_), Some(msg)) => {
                if self.length() <= min || self.length() >= max {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (Some(_), Some(_), None) => {
                if self.length() <= min || self.length() >= max {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (Some(_), None, Some(msg)) => {
                if self.length() <= min {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (Some(_), None, None) => {
                if self.length() <= min {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (None, Some(_), Some(msg)) => {
                if self.length() >= max {
                    return Err(crate::error::Error::Custom(msg));
                }
            }
            (None, Some(_), None) => {
                if self.length() >= max {
                    return Err(crate::error::Error::InvalidLength { min, max });
                }
            }
            (None, None, None) | (None, None, Some(_)) => (),
        }

        Ok(())
    }

    fn length(&self) -> Option<usize>;
}

macro_rules! validate_type_with_deref {
    ($type:ty) => {
        impl<T: ValidateLength> ValidateLength for $type {
            fn length(&self) -> Option<usize> {
                T::length(self)
            }
        }
    };
}

validate_type_with_deref!(Box<T>);
validate_type_with_deref!(Arc<T>);
validate_type_with_deref!(Rc<T>);
validate_type_with_deref!(Ref<'_, T>);
validate_type_with_deref!(RefMut<'_, T>);

macro_rules! validate_type_with_len {
    ($type:ty, $($generic:ident),*) => {
        impl<$($generic),*> ValidateLength for $type {
            fn length(&self) -> Option<usize> {
                Some(self.len())
            }
        }
    };
}

validate_type_with_len!([T], T);
validate_type_with_len!(Vec<T>, T);
validate_type_with_len!(VecDeque<T>, T);
validate_type_with_len!(HashSet<T>, T);
validate_type_with_len!(BTreeSet<T>, T);
validate_type_with_len!(HashMap<K, V>, K, V);
validate_type_with_len!(BTreeMap<K, V>, K, V);

macro_rules! validate_type_with_chars {
    ($type:ty) => {
        impl ValidateLength for $type {
            fn length(&self) -> Option<usize> {
                Some(self.chars().count())
            }
        }
    };
}

validate_type_with_chars!(str);
validate_type_with_chars!(&str);
validate_type_with_chars!(String);

impl<T: ValidateLength> ValidateLength for Option<T> {
    fn length(&self) -> Option<usize> {
        let Some(s) = self else {
            return None;
        };

        T::length(s)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidateLength;

    #[test]
    fn test_validate_str_length() {
        let valid = "hello".validate_length(Some(1), Some(10), None).unwrap();
        assert_eq!(valid, ());
    }

    #[test]
    fn test_validate_vec_length() {
        let valid = vec![1, 2, 3]
            .validate_length(Some(1), Some(10), None)
            .unwrap();
        assert_eq!(valid, ());
    }
}
