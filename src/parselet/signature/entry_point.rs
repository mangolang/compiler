use crate::common::codeparts::name::Name;
use crate::parselet::body::BodyParselet;

#[derive(Debug)]
pub struct EntryPointParselet {
    name: Name,
    body: BodyParselet,
}
