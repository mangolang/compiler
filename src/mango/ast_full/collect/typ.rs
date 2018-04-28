use mango::util::encdec::ToText;
use std::fmt::Debug;
use std::hash::Hash;

/// AST trait to be used for AST nodes.
pub trait AST: PartialEq + Eq + Hash + Debug + ToText {}
