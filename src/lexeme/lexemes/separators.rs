use ::std::hash;

use crate::io::slice::{SourceLocation, SourceSlice};

macro_rules! implement_separator {
    ($name: ident) => {

        impl $name {
            pub fn new(source: SourceSlice) -> Self {
                $name { source}
            }
        }

        impl SourceLocation for $name {
            fn source(&self) -> &SourceSlice {
                &self.source
            }
        }

        impl PartialEq for $name {
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }

        impl hash::Hash for $name {
            fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct ColonLexeme {
    source: SourceSlice,
}
implement_separator!(ColonLexeme);

#[derive(Debug, Eq, Clone)]
pub struct CommaLexeme {
    source: SourceSlice,
}
implement_separator!(CommaLexeme);

#[derive(Debug, Eq, Clone)]
pub struct EllipsisLexeme {
    source: SourceSlice,
}
implement_separator!(EllipsisLexeme);

#[derive(Debug, Eq, Clone)]
pub struct PeriodLexeme {
    source: SourceSlice,
}
implement_separator!(PeriodLexeme);

#[derive(Debug, Eq, Clone)]
pub struct NewlineLexeme {
    source: SourceSlice,
}
implement_separator!(NewlineLexeme);
