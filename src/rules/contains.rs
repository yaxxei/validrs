use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use crate::error::{Error, Result};

pub trait ValidateContains<'a, T: PartialEq> {
    fn validate_contains(&'a self, values: &'a [T], msg: Option<String>) -> Result<()> {
        if !values.iter().all(|v| self.contains_value(v)) {
            return match msg {
                Some(msg) => Err(Error::Custom(msg.into())),
                None => Err(Error::DoNotContains),
            };
        }
        Ok(())
    }

    fn contains_value(&self, value: &T) -> bool;
}

// Для проверки подстрок в &str
impl<'a> ValidateContains<'a, &'a str> for str {
    fn contains_value(&self, value: &&'a str) -> bool {
        self.contains(*value)
    }
}

// Для проверки подстрок в String
impl<'a> ValidateContains<'a, &'a str> for String {
    fn contains_value(&self, value: &&'a str) -> bool {
        self.as_str().contains(*value)
    }
}

// Для проверки символов в &str
impl ValidateContains<'_, char> for str {
    fn contains_value(&self, value: &char) -> bool {
        self.contains(*value)
    }
}

// Для проверки символов в String
impl ValidateContains<'_, char> for String {
    fn contains_value(&self, value: &char) -> bool {
        self.as_str().contains(*value)
    }
}

macro_rules! validate_type_with_contains {
    ($type:ty) => {
        impl<T: PartialEq + Eq + std::hash::Hash + Ord> ValidateContains<'_, T> for $type {
            fn contains_value(&self, value: &T) -> bool {
                self.contains(value)
            }
        }
    };
}

validate_type_with_contains!([T]);
validate_type_with_contains!(Vec<T>);
validate_type_with_contains!(VecDeque<T>);
validate_type_with_contains!(HashSet<T>);
validate_type_with_contains!(BTreeSet<T>);

macro_rules! validate_type_with_contains_keys {
    ($type:ty, $($generic:ident),*) => {
        impl<K: PartialEq, V> ValidateContains<'_, K> for $type {
            fn contains_value(&self, value: &K) -> bool {
                self.keys().collect::<Vec<_>>().contains(&value)
            }
        }
    };
}

validate_type_with_contains_keys!(HashMap<K, V>, K, V);
validate_type_with_contains_keys!(BTreeMap<K, V>, K, V);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::ValidateContains;

    #[test]
    fn test_validate_contains_substring_in_str() {
        let valid = "test@gmail.com".validate_contains(&["@"], None).unwrap();
        assert_eq!(valid, ());
    }

    #[test]
    fn test_validate_contains_substring_in_string() {
        let valid = "test@gmail.com"
            .to_string()
            .validate_contains(&["@"], None)
            .unwrap();
        assert_eq!(valid, ());
    }

    #[test]
    fn test_validate_contains_element_in_vec() {
        let valid = vec!["abc", "def", "ghi"]
            .validate_contains(&["def"], None)
            .unwrap();
        assert_eq!(valid, ());
    }

    #[test]
    fn test_validate_contains_element_in_hashmap() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");
        let valid = map.validate_contains(&[1], None).unwrap();
        assert_eq!(valid, ());
    }
}
