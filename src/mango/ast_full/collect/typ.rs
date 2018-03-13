use mango::util::encdec::ToText;

/// Trait to be implemented by everything in the full abstract syntax tree.
//pub trait AST: ToText + ToObjectNotation {  // todo: add ON again later
pub trait AST: ToText {}
