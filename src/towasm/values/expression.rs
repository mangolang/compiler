use crate::towasm::collect::Type;
use crate::towasm::Wasm;

/// A (combination of) operations that has an output value
pub trait Expression: Wasm {
    fn typ(&self) -> &Type;

    //    Const(Const),
    //    Local(GetLocal),
    //    Mul(Mul),
    //    Add(Add),
    //    Gt(Gt),
    //    Lt(Lt),
}

//impl Expression {
//    pub fn typ(&self) -> &Type {
//        match self {
//            Expression::Const(op) => &op.typ,
//            Expression::Local(op) => &op.typ(),
//            Expression::Mul(op) => &op.typ(),
//            Expression::Add(op) => &op.typ(),
//            Expression::Gt(op) => &op.typ(),
//            Expression::Lt(op) => &op.typ(),
//        }
//    }
//}
