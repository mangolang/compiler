use mango::util::encdec::ToText;
use mango::util::encdec::ToObjectNotation;

/// Trait to be implemented by everything in the full abstract syntax tree.
pub trait AST: ToText + ToObjectNotation {
}
