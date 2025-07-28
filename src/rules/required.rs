use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use crate::error::{Error, Result};

pub trait ValidateRequired {
    fn validate_required(&self, msg: Option<String>) -> Result<()> {
        if self.empty() {
            return match msg {
                Some(msg) => Err(Error::Custom(msg)),
                None => Err(Error::Required),
            };
        }
        Ok(())
    }

    fn empty(&self) -> bool;
}

macro_rules! validate_type_with_is_empty {
    ($type:ty) => {
        impl ValidateRequired for $type {
            fn empty(&self) -> bool {
                self.is_empty()
            }
        }
    };

    ($type:ty, $($generic:ident),*) => {
        impl<$($generic),*> ValidateRequired for $type {
            fn empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

validate_type_with_is_empty!(str);
validate_type_with_is_empty!(&str);
validate_type_with_is_empty!(String);
validate_type_with_is_empty!([T], T);
validate_type_with_is_empty!(Vec<T>, T);
validate_type_with_is_empty!(VecDeque<T>, T);
validate_type_with_is_empty!(HashSet<T>, T);
validate_type_with_is_empty!(BTreeSet<T>, T);
validate_type_with_is_empty!(HashMap<K, V>, K, V);
validate_type_with_is_empty!(BTreeMap<K, V>, K, V);

impl<T> ValidateRequired for Option<T> {
    fn empty(&self) -> bool {
        self.is_none()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::ValidateRequired;

    #[test]
    fn test_validate_option() {
        let valid = Some(1).validate_required(None).unwrap();
        assert_eq!(valid, ())
    }

    #[test]
    fn test_validate_str() {
        let valid = "abc".validate_required(None).unwrap();
        assert_eq!(valid, ())
    }

    #[test]
    fn test_validate_vec() {
        let valid = vec![1].validate_required(None).unwrap();
        assert_eq!(valid, ())
    }

    #[test]
    fn test_validate_hashmap() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        let valid = map.validate_required(None).unwrap();
        assert_eq!(valid, ())
    }
}
