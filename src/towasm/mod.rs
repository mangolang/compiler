// TODO: possibly extract this to a separate crate

pub use typ::Wasm;

pub use self::control::*;
pub use self::numeric::*;
pub use self::scope::*;
pub use self::values::*;

pub mod collect;
pub mod typ;

pub mod control;
pub mod numeric;
pub mod scope;
//pub mod util;
//pub use self::util::*;

#[cfg(test)]
mod tests;
pub mod values;
