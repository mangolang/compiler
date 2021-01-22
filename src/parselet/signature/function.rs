use crate::common::codeparts::name::Name;
use crate::parselet::body::function::FunctionBodyParselet;

#[derive(Debug)]
pub struct FunctionParselet {
    name: Name,
    body: FunctionBodyParselet,
}
