use crate::parsing::util::cursor::End;

pub mod cursor;

#[derive(Debug, PartialEq, Eq)]
pub struct NoMatch;

pub type ParseRes<'a, T> = Result<(cursor::ParseCursor<'a>, T), NoMatch>;

impl From<End> for NoMatch {
    fn from(_: End) -> Self {
        NoMatch
    }
}
