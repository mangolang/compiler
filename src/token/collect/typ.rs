use ::std::fmt::Debug;
use ::std::hash::Hash;

use crate::util::encdec::ToText;

/// Token trait to be implemented by all lexed tokens.
pub trait Token: PartialEq + Eq + Hash + Debug + Clone {
    //fn source_slice(&self) -> SourceSlice;
}
