use mango::token::special::UnlexableToken;
use mango::token::tokens::AssociationToken;
use mango::token::tokens::EndStatementToken;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::KeywordToken;
use mango::token::tokens::LiteralToken;
use mango::token::tokens::OperatorToken;
use mango::token::tokens::ParenthesisCloseToken;
use mango::token::tokens::ParenthesisOpenToken;
use mango::util::encdec::ToText;

/// Collection of all possible tokens.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Tokens {
    Association(AssociationToken),
    Identifier(IdentifierToken),
    Keyword(KeywordToken),
    Literal(LiteralToken),
    Operator(OperatorToken),
    ParenthesisOpen(ParenthesisOpenToken),
    ParenthesisClose(ParenthesisCloseToken),
    EndStatement(EndStatementToken),
    Unlexable(UnlexableToken),
}

impl ToText for Tokens {
    fn to_text(&self) -> String {
        // todo: this seems inefficient, and code bloat
        // todo: but perhaps good for parsing, hard to get type back from trait obj
        use self::Tokens::*;
        match self {
            Association(token) => token.to_text(),
            Identifier(token) => token.to_text(),
            Keyword(token) => token.to_text(),
            Literal(token) => token.to_text(),
            Operator(token) => token.to_text(),
            ParenthesisOpen(token) => token.to_text(),
            ParenthesisClose(token) => token.to_text(),
            EndStatement(token) => token.to_text(),
            Unlexable(token) => token.to_text(),
        }
    }
}
