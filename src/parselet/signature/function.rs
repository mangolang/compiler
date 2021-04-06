use crate::common::codeparts::name::Name;
use crate::parselet::body::function::FunctionBodyParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionParselet {
    name: Name,
    body: FunctionBodyParselet,
}
