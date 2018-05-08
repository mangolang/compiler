use mango::towasm::values::DeclareLocal;

pub enum Statement {
    Local(DeclareLocal),
}
