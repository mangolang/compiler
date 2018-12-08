/// This infrastructure is for generating identifiers for constructs that are anonymous
/// There are several requirements:
/// * Names can repeat for sibling scopes, but no shadowing of parents.
/// * Things should have exactly one name.
/// * Names, anonymous or otherwise, should not conflict.
/// * It should be possible to provide prefixes.
/// * Anonymous names should be short.
/// * Parent scope should be able to resolve new names after the child.
///
///

//TODO @mark: note that there are two phases:
// * Encountering names, new and existing -> look up prefix and return identifier; namepool growing and degenerate, names unknown
// * Compiling and code generation -> look up identifier and get full name; namepool static and unique

mod name;
pub use self::name::{KnownName, Name, PendingName, RawName};

pub mod pool;
