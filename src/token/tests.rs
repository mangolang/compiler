use crate::token::collect::MemoryTokenStream;
use crate::token::tokens::AssociationToken;
use crate::token::tokens::EndStatementToken;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::KeywordToken;
use crate::token::tokens::LiteralToken;
use crate::util::strtype::Name;
use crate::util::strtype::StrType;

#[test]
fn test_tokens_eq() {
    use super::Tokens::*;
    let my_var = Name::new("my_var").unwrap();
    let tokens = MemoryTokenStream::new(vec![
        Keyword(KeywordToken::from_str("let").unwrap()),
        Keyword(KeywordToken::from_str("mut").unwrap()),
        Identifier(IdentifierToken::from_name(my_var)),
        Association(AssociationToken::from_unprefixed()),
        Literal(LiteralToken::Int(21)),
        EndStatement(EndStatementToken::new_semicolon()),
        Identifier(IdentifierToken::from_name(my_var)),
        Association(AssociationToken::from_str("+").unwrap()),
        Identifier(IdentifierToken::from_name(my_var)),
    ]);
    assert_eq!(tokens, tokens);
    assert_ne!(tokens, MemoryTokenStream::new(vec![]));
}
