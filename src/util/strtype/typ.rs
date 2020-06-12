use ::std::fmt;
use ::std::hash::Hash;

use crate::common::error::MsgResult;
use crate::util::strtype::Msg;

/// A trait for types that wrap a string matching a certain structure.
pub trait StrType: Sized + fmt::Display + Hash + PartialEq<Self> + Eq {
    /// Validate whether this is a valid string for this type. Returns an explanation message if not.
    fn validate(value: &str) -> MsgResult<()>;

    /// Constructor that creates an instance if valid, or a validation message if invalid.
    fn new<S: Into<String>>(txt: S) -> MsgResult<Self>;

    /// Constructor that creates an instance if valid, or a validation message if invalid, by copying a string reference.
    fn copy_new(txt: &str) -> MsgResult<Self> {
        Self::new(txt.to_string())
    }

    /// Alternative constructor that panics on invalid input.
    fn from_valid(txt: &str) -> Self {
        Self::copy_new(txt).unwrap()
    }
}
