/// This infrastructure is for generating identifiers for constructs that are anonymous
/// There are several requirements:
/// * There could be scoping, with no shadowing of outer scopes.
/// * Things should have exactly one name.
/// * Names, anonymous or otherwise, should not conflict.
/// * It should be possible to provide prefixes.
/// * Anonymous names should be short.
mod name;
pub use self::name::{KnownName, Name, PendingName, RawName};

mod pool;
pub use self::pool::NamePool;
