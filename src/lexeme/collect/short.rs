use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::lexeme::{
    AssociationToken, EndBlockToken, EndStatementToken, IdentifierToken, KeywordToken, LiteralToken, OperatorToken, ParenthesisCloseToken,
    ParenthesisOpenToken, StartBlockToken, Tokens, UnlexableToken,
};
use crate::lexeme::brackets::{BracketCloseToken, BracketOpenToken};
use crate::lexeme::separators::ColonToken;
use crate::lexeme::tokens::separators::{CommaToken, EllipsisToken, NewlineToken, PeriodToken};
use crate::util::codeparts::Keyword;
use crate::util::numtype::f64eq;

//TODO @mark: replace more lexeme usages by short versions

pub fn association(txt: &str) -> MsgResult<Tokens> {
    Ok(Tokens::Association(AssociationToken::from_str(txt)?))
}

pub fn identifier(txt: &str) -> MsgResult<Tokens> {
    Ok(Tokens::Identifier(IdentifierToken::from_str(txt)?))
}

/// Parse a keyword, including reserved keywords for future use.
pub fn keyword_or_reserved(txt: &str) -> MsgResult<Tokens> {
    Ok(Tokens::Keyword(KeywordToken::from_str(txt)?))
}

/// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
pub fn keyword_supported(txt: &str) -> MsgResult<Tokens> {
    match Keyword::from_str(txt)? {
        Keyword::Reserved(word) => Err(ErrMsg::new(format!("Keyword '{}' not implemented", word))),
        kw => Ok(Tokens::Keyword(KeywordToken::from_keyword(kw))),
    }
}

pub fn literal_text(txt: impl Into<String>) -> Tokens {
    Tokens::Literal(LiteralToken::Text(txt.into()))
}

pub fn literal_bool(b: bool) -> Tokens {
    Tokens::Literal(LiteralToken::Boolean(b))
}

pub fn literal_int(nr: i64) -> Tokens {
    Tokens::Literal(LiteralToken::Int(nr))
}

pub fn literal_real(nr: impl Into<f64eq>) -> Tokens {
    Tokens::Literal(LiteralToken::Real(nr.into()))
}

pub fn operator(txt: &str) -> MsgResult<Tokens> {
    Ok(Tokens::Operator(OperatorToken::from_str(&txt)?))
}

pub fn parenthesis_open() -> Tokens {
    Tokens::ParenthesisOpen(ParenthesisOpenToken::new())
}

pub fn parenthesis_close() -> Tokens {
    Tokens::ParenthesisClose(ParenthesisCloseToken::new())
}

pub fn bracket_open() -> Tokens {
    Tokens::BracketOpen(BracketOpenToken::new())
}

pub fn bracket_close() -> Tokens {
    Tokens::BracketClose(BracketCloseToken::new())
}

// pub fn end_statement() -> Tokens {
//     //TODO @mark: for now only create newlines
//     Tokens::EndStatement(EndStatementToken::new_end_line())
// }

pub fn start_block() -> Tokens {
    Tokens::StartBlock(StartBlockToken::new())
}

pub fn end_block() -> Tokens {
    Tokens::EndBlock(EndBlockToken::new2())
}

pub fn colon() -> Tokens {
    Tokens::Colon(ColonToken::new())
}
pub fn comma() -> Tokens {
    Tokens::Comma(CommaToken::new())
}
pub fn ellipsis() -> Tokens {
    Tokens::Ellipsis(EllipsisToken::new())
}
pub fn period() -> Tokens {
    Tokens::Period(PeriodToken::new())
}
pub fn newline() -> Tokens {
    Tokens::Newline(NewlineToken::new())
}

pub fn unlexable(txt: impl Into<String>) -> Tokens {
    Tokens::Unlexable(UnlexableToken::new(txt.into()))
}
