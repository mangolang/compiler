use crate::io::slice::SourceSlice;
use crate::lexeme::identifier::SimpleIdentifierLexeme;

//TODO @mark: keep these representations? or e.g. `()` or `void` or `null`?
//TODO @mark: should these be special-cased somewhere? at least registered as known identifiers in every file.
const VOID_VALUE: &str = "none";
const VOID_TYPE: &str = "None";

#[derive(Debug, PartialEq, Eq)]
pub struct TypeParselet {
    //TODO @mark: this is temporary structure
    tmp_type: SimpleIdentifierLexeme,
}

impl TypeParselet {
    pub fn new(typ: SimpleIdentifierLexeme) -> Self {
        TypeParselet { tmp_type: typ }
    }

    pub fn void(source: SourceSlice) -> Self {
        Self::new(SimpleIdentifierLexeme::from_valid(VOID_TYPE, source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_void() {
        let void = TypeParselet::void(SourceSlice::mock());
        assert_eq!(void.tmp_type.name.as_str(), "None");
    }
}
