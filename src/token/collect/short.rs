use ::std::str::FromStr;

use crate::common::error::MangoResult;
use crate::token::{AssociationToken, EndBlockToken, EndStatementToken, IdentifierToken, KeywordToken, LiteralToken, OperatorToken, ParenthesisCloseToken, ParenthesisOpenToken, StartBlockToken, Tokens, UnlexableToken};
use crate::util::numtype::f64eq;

pub fn association(txt: &str) -> MangoResult<Tokens> {
    Ok(Tokens::Association(AssociationToken::from_str(txt)?))
}

pub fn identifier(txt: &str) -> MangoResult<Tokens> {
    Ok(Tokens::Identifier(IdentifierToken::from_str(txt)?))
}

pub fn keyword(txt: &str) -> MangoResult<Tokens> {
    Ok(Tokens::Keyword(KeywordToken::from_str(txt)?))
}

pub fn literal_text(txt: impl Into<String>) -> Tokens {
    Tokens::Literal(LiteralToken::Text(txt.into()))
}

pub fn literal_int(nr: i64) -> Tokens {
    Tokens::Literal(LiteralToken::Int(nr))
}

pub fn literal_real(nr: f64eq) -> Tokens {
    Tokens::Literal(LiteralToken::Real(nr))
}

pub fn operator(txt: &str) -> MangoResult<Tokens> {
    Ok(Tokens::Operator(OperatorToken::from_str(&txt)?))
}

pub fn parenthesis_open() -> Tokens {
    Tokens::ParenthesisOpen(ParenthesisOpenToken::new())
}

pub fn parenthesis_close() -> Tokens {
    Tokens::ParenthesisClose(ParenthesisCloseToken::new())
}

pub fn end_statement() -> Tokens {
    //TODO @mark: for now only create newlines
    Tokens::EndStatement(EndStatementToken::new_end_line())
}

pub fn start_block() -> Tokens {
    Tokens::StartBlock(StartBlockToken::new())
}

pub fn end_block() -> Tokens {
    Tokens::EndBlock(EndBlockToken::new2())
}

pub fn unlexable(txt: impl Into<String>) -> Tokens {
    Tokens::Unlexable(UnlexableToken::new(txt.into()))
}
