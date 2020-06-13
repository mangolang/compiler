use crate::util::encdec::ToText;
use crate::token::Token;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CommaToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EllipsisToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PeriodToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct NewlineToken {}

impl CommaToken {
    fn new() -> Self {
        CommaToken {}
    }
}
impl EllipsisToken {
    fn new() -> Self {
        EllipsisToken {}
    }
}
impl PeriodToken {
    fn new() -> Self {
        PeriodToken {}
    }
}
impl NewlineToken {
    fn new() -> Self {
        NewlineToken {}
    }
}

impl Token for CommaToken {}

impl Token for EllipsisToken {}

impl Token for PeriodToken {}

impl Token for NewlineToken {}
