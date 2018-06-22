use mango::token::collect::MemoryTokenStream;
use mango::token::tokens::AssociationToken;
use mango::token::tokens::EndStatementToken;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::KeywordToken;
use mango::token::tokens::LiteralToken;
use mango::util::strtype::Name;
use mango::util::strtype::StrType;

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
