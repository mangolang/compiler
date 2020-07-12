use crate::util::encdec::ToText;
use crate::token::Token;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ColonToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CommaToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EllipsisToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PeriodToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct NewlineToken {}

impl ColonToken {
    pub fn new() -> Self {
        ColonToken {}
    }
}

impl CommaToken {
    pub fn new() -> Self {
        CommaToken {}
    }
}

impl EllipsisToken {
    pub fn new() -> Self {
        EllipsisToken {}
    }
}

impl PeriodToken {
    pub fn new() -> Self {
        PeriodToken {}
    }
}

impl NewlineToken {
    pub fn new() -> Self {
        NewlineToken {}
    }
}

impl Token for ColonToken {}

impl Token for CommaToken {}

impl Token for EllipsisToken {}

impl Token for PeriodToken {}

impl Token for NewlineToken {}
