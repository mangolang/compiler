use std::fmt;
use std::hash::Hash;
use mango::util::strtype::Msg;

/// A trait for types that wrap a string matching a certain structure.
pub trait StrType: Sized + fmt::Display + Hash + PartialEq<Self> + Eq {

    /// Validate whether this is a valid string for this type. Returns an explanation message if not.
    //todo: public
    fn validate(value: &str) -> Result<&str, Msg>;

    /// Constructor that creates an instance if valid, or a validation message if invalid.
    fn new<S>(txt: S) -> Result<Self, Msg> where S: ToString;

    /// Alternative constructor that panics on invalid input.
    fn from_valid<S>(txt: S) -> Self where S: ToString {
        return Self::new(txt.to_string()).unwrap();
    }
}
