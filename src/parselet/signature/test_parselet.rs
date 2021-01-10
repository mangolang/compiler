use crate::common::codeparts::name::Name;
use crate::parselet::body::BodyParselet;

#[derive(Debug)]
pub struct TestParselet {
    name: Name,
    body: BodyParselet,
}
