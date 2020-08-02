use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ColonLexeme {
    source: SourceSlice,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CommaLexeme {
    source: SourceSlice,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EllipsisLexeme {
    source: SourceSlice,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PeriodLexeme {
    source: SourceSlice,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct NewlineLexeme {
    source: SourceSlice,
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
