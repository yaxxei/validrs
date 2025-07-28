pub mod alphanumeric;
pub mod alphapetic;
pub mod ascii;
pub mod color;
pub mod contains;
pub mod email;
pub mod ip;
pub mod length;
pub mod lowercase;
pub mod negative;
pub mod phone;
pub mod positive;
pub mod range;
pub mod regex;
pub mod required;
pub mod uppercase;
// pub mod digit;
// pub mod hexdigit;
// pub mod octdigit;
// pub mod punctuation;
// pub mod graphic;
// pub mod whitespace;
// pub mod control;

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for &str {
    fn as_str(&self) -> &str {
        self
    }
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}

impl AsStr for std::borrow::Cow<'_, str> {
    fn as_str(&self) -> &str {
        std::borrow::Cow::as_ref(self)
    }
}
