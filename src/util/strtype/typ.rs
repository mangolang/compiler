use ::std::borrow::Cow;
use ::std::fmt;
use ::std::hash;

use crate::common::error::MsgResult;

/// A trait for types that wrap a string matching a certain structure.
pub trait StrType: Sized + fmt::Display + hash::Hash + PartialEq<Self> + Eq {
    /// Validate whether this is a valid string for this type. Returns an explanation message if not.
    fn validate(value: &str) -> MsgResult<()>;

    /// Constructor that creates an instance if valid, or a validation message if invalid.
    fn new<'a>(txt: impl AsRef<str>) -> MsgResult<Self>;

    /// Alternative constructor that panics on invalid input.
    fn from_valid(txt: &str) -> Self {
        Self::new(Cow::from(txt)).unwrap()
    }
}
