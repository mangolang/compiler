use crate::util::encdec::ToText;
use std::fmt::Debug;
use std::hash::Hash;

/// Token trait to be implemented by all lexed tokens.
pub trait Token: PartialEq + Eq + Hash + Debug + ToText + Clone {
    //fn source_slice(&self) -> SourceSlice;
}
