use crate::common::codeparts::name::Name;
use crate::parselet::signature::function::FunctionParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct TestParselet {
    name: Name,
    body: FunctionParselet,
    //TODO @mark: chagne to test parselet
}
