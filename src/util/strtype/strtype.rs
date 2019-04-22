use crate::util::strtype::Msg;
use std::fmt;
use std::hash::Hash;

/// A trait for types that wrap a string matching a certain structure.
pub trait StrType: Sized + fmt::Display + Hash + PartialEq<Self> + Eq {
    /// Validate whether this is a valid string for this type. Returns an explanation message if not.
    fn validate(value: &str) -> Result<(), Msg>;

    /// Constructor that creates an instance if valid, or a validation message if invalid.
    fn new(txt: String) -> Result<Self, Msg>;

    /// Constructor that creates an instance if valid, or a validation message if invalid, by copying a string reference.
    fn copy_new(txt: &str) -> Result<Self, Msg> {
        Self::new(txt.to_string())
    }

    /// Alternative constructor that panics on invalid input.
    fn from_valid(txt: &str) -> Self {
        Self::copy_new(txt).unwrap()
    }
}
