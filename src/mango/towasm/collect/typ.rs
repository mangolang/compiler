use mango::util::encdec::ToText;
use mango::util::encdec::ToCode;

/// WASM node
//pub trait WASM: PartialEq + Eq + Hash + Debug + ToText {}
pub trait Wasm: ToText + ToCode {}
