pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid length: min = {min:?}, max = {max:?}")]
    InvalidLength {
        min: Option<usize>,
        max: Option<usize>,
    },

    #[error("Do not contains")]
    DoNotContains, /*  { values: Vec<String> } */

    #[error("Field must be required")]
    Required,

    #[error("Number must be negative")]
    Negative,

    #[error("Number must be positive")]
    Positive,

    #[error("Email is invalid")]
    Email,

    #[error("Ip is invalid")]
    Ip,

    #[error("String is not alphanumeric")]
    Alphanumeric,

    #[error("String is not alphabetic")]
    Alphabetic,

    #[error("String is not ascii")]
    Ascii,

    #[error("String is not lowercase")]
    Lowercase,

    #[error(transparent)]
    Color(#[from] crate::rules::color::ColorError),

    #[error(transparent)]
    Phone(#[from] crate::rules::phone::PhoneError),

    #[error("{0}")]
    Custom(String),
}

#[macro_export]
macro_rules! validate_error {
    ($result:expr, $msg:expr, $default:expr) => {
        match $result {
            Ok(_) => (),
            Err(_) => return Err($msg.map(Error::Custom).unwrap_or($default)),
        }
    };
    ($condition:expr, $msg:expr, $default:expr) => {
        if !$condition {
            return Err($msg.map(Error::Custom).unwrap_or($default));
        }
    };
}
