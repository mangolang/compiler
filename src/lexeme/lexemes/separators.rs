use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ColonLexeme {
    source: SourceSlice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CommaLexeme {
    source: SourceSlice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct EllipsisLexeme {
    source: SourceSlice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PeriodLexeme {
    source: SourceSlice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NewlineLexeme {
    source: SourceSlice,
}

impl ColonLexeme {
    pub fn new(source: SourceSlice) -> Self {
        ColonLexeme { source}
    }
}

impl CommaLexeme {
    pub fn new(source: SourceSlice) -> Self {
        CommaLexeme { source}
    }
}

impl EllipsisLexeme {
    pub fn new(source: SourceSlice) -> Self {
        EllipsisLexeme { source}
    }
}

impl PeriodLexeme {
    pub fn new(source: SourceSlice) -> Self {
        PeriodLexeme { source}
    }
}

impl NewlineLexeme {
    pub fn new(source: SourceSlice) -> Self {
        NewlineLexeme { source}
    }
}

impl SourceLocation for ColonLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for CommaLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for EllipsisLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for PeriodLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for NewlineLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}
