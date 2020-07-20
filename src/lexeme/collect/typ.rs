use ::std::fmt::Debug;
use ::std::hash::Hash;

use crate::util::encdec::ToText;

/// Lexeme trait to be implemented by all lexed lexemes.
pub trait Lexeme: PartialEq + Eq + Hash + Debug + Clone {
    //fn source_slice(&self) -> SourceSlice;
}
