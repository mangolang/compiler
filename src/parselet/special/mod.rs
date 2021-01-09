pub use self::unparseable::UnparseableParselet;
use smallvec::SmallVec;
use crate::parselet::ExpressionParselets;

mod unparseable;

pub type GroupType = SmallVec<[ExpressionParselets; 3]>;

