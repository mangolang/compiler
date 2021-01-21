use crate::common::codeparts::name::Name;

#[derive(Debug)]
pub struct TestParselet {
    name: Name,
    body: BodyParselet,
}
