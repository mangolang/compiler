use crate::parsing::util::cursor::End;

pub mod cursor;

#[derive(Debug, PartialEq, Eq)]
pub struct NoMatch;

pub type ParseRes<T> = Result<T, NoMatch>;

impl From<End> for NoMatch {
    fn from(_: End) -> Self {
        NoMatch
    }
}
