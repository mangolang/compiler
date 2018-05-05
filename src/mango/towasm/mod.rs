
// TODO: possibly extract this to a separate crate

pub mod collect;
pub use self::collect::IWasm;

pub mod numeric;
pub use self::numeric::*;

pub mod control;
pub use self::control::*;

pub mod scope;
pub use self::scope::*;

#[cfg(test)]
mod tests;

