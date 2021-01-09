use std::str::FromStr;

use crate::lexeme::collect::MemoryLexemeStream;
use crate::lexeme::lexemes::AssociationLexeme;
use crate::lexeme::lexemes::EndStatementLexeme;
use crate::lexeme::lexemes::IdentifierLexeme;
use crate::lexeme::lexemes::KeywordLexeme;
use crate::lexeme::lexemes::LiteralLexeme;
use crate::util::strtype::Name;
use crate::util::strtype::StrType;

#[test]
fn test_lexemes_eq() {
    use super::Lexemes::*;
    let my_var = Name::new("my_var").unwrap();
    let lexemes = MemoryLexemeStream::new(vec![
        Keyword(KeywordLexeme::from_str("let").unwrap()),
        Keyword(KeywordLexeme::from_str("mut").unwrap()),
        Identifier(IdentifierLexeme::from_name(my_var)),
        Association(AssociationLexeme::from_unprefixed()),
        Literal(LiteralLexeme::Int(21)),
        EndStatement(EndStatementLexeme::new_semicolon()),
        Identifier(IdentifierLexeme::from_name(my_var)),
        Association(AssociationLexeme::from_str("+").unwrap()),
        Identifier(IdentifierLexeme::from_name(my_var)),
    ]);
    assert_eq!(lexemes, lexemes);
    assert_ne!(lexemes, MemoryLexemeStream::new(vec![]));
}
