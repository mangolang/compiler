use crate::io::slice::SourceSlice;
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::FQIdentifierLexeme;
use crate::parselet::files::import::ImportParselet;

pub fn import(fqn: impl AsRef<str>) -> ImportParselet {
    ImportParselet::new(FQIdentifierLexeme::from_str(fqn.as_ref(), SourceSlice::mock()).unwrap(), None)
}

pub fn import_alias(fqn: impl AsRef<str>, alias: impl AsRef<str>) -> ImportParselet {
    ImportParselet::new(
        FQIdentifierLexeme::from_str(fqn.as_ref(), SourceSlice::mock()).unwrap(),
        Some(SimpleIdentifierLexeme::from_str(alias.as_ref(), SourceSlice::mock()).unwrap()),
    )
}
