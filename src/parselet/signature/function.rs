use crate::common::codeparts::name::Name;
use crate::parselet::body::code_body::CodeBodyParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionParselet {
    name: Name,
    body: CodeBodyParselet,
}
