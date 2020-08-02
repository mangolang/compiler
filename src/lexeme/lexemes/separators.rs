use crate::lexeme::Lexeme;
use crate::util::encdec::ToText;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ColonLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CommaLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EllipsisLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PeriodLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct NewlineLexeme {}

impl ColonLexeme {
    pub fn new() -> Self {
        ColonLexeme {}
    }
}

impl CommaLexeme {
    pub fn new() -> Self {
        CommaLexeme {}
    }
}

impl EllipsisLexeme {
    pub fn new() -> Self {
        EllipsisLexeme {}
    }
}

impl PeriodLexeme {
    pub fn new() -> Self {
        PeriodLexeme {}
    }
}

impl NewlineLexeme {
    pub fn new() -> Self {
        NewlineLexeme {}
    }
}
