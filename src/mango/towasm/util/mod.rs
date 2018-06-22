/// This infrastructure is for generating identifiers for constructs that are anonymous
/// There are several requirements:
/// * There could be scoping, with no shadowing of outer scopes.
/// * Things should have exactly one name.
/// * Names, anonymous or otherwise, should not conflict.
/// * It should be possible to provide prefixes.
/// * Anonymous names should be short.

mod name;
pub use self::name::Name;

mod given_name;
pub use self::given_name::GivenName;

mod anonymous;
pub use self::anonymous::AnonymousName;
