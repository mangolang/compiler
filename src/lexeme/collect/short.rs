use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::lexeme::{
    AssociationLexeme, EndBlockLexeme, EndStatementLexeme, IdentifierLexeme, KeywordLexeme, LiteralLexeme, OperatorLexeme, ParenthesisCloseLexeme,
    ParenthesisOpenLexeme, StartBlockLexeme, Lexeme, UnlexableLexeme,
};
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::util::codeparts::Keyword;
use crate::util::numtype::f64eq;

//TODO @mark: replace more lexeme usages by short versions

pub fn association(txt: &str) -> MsgResult<Lexeme> {
    Ok(Lexeme::Association(AssociationLexeme::from_str(txt)?))
}

pub fn identifier(txt: &str) -> MsgResult<Lexeme> {
    Ok(Lexeme::Identifier(IdentifierLexeme::from_str(txt)?))
}

/// Parse a keyword, including reserved keywords for future use.
pub fn keyword_or_reserved(txt: &str) -> MsgResult<Lexeme> {
    Ok(Lexeme::Keyword(KeywordLexeme::from_str(txt)?))
}

/// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
pub fn keyword_supported(txt: &str) -> MsgResult<Lexeme> {
    match Keyword::from_str(txt)? {
        Keyword::Reserved(word) => Err(ErrMsg::new(format!("Keyword '{}' not implemented", word))),
        kw => Ok(Lexeme::Keyword(KeywordLexeme::from_keyword(kw))),
    }
}

pub fn literal_text(txt: impl Into<String>) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Text(txt.into()))
}

pub fn literal_int(nr: i64) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Int(nr))
}

pub fn literal_real(nr: impl Into<f64eq>) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Real(nr.into()))
}

pub fn literal_bool(b: bool) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Boolean(b))
}

pub fn operator(txt: &str) -> MsgResult<Lexeme> {
    Ok(Lexeme::Operator(OperatorLexeme::from_str(&txt)?))
}

pub fn parenthesis_open() -> Lexeme {
    Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new())
}

pub fn parenthesis_close() -> Lexeme {
    Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new())
}

pub fn bracket_open() -> Lexeme {
    Lexeme::BracketOpen(BracketOpenLexeme::new())
}

pub fn bracket_close() -> Lexeme {
    Lexeme::BracketClose(BracketCloseLexeme::new())
}

// pub fn end_statement() -> Lexemes {
//     //TODO @mark: for now only create newlines
//     Lexemes::EndStatement(EndStatementLexeme::new_end_line())
// }

pub fn start_block() -> Lexeme {
    Lexeme::StartBlock(StartBlockLexeme::new())
}

pub fn end_block() -> Lexeme {
    Lexeme::EndBlock(EndBlockLexeme::new2())
}

pub fn colon() -> Lexeme {
    Lexeme::Colon(ColonLexeme::new())
}
pub fn comma() -> Lexeme {
    Lexeme::Comma(CommaLexeme::new())
}
pub fn ellipsis() -> Lexeme {
    Lexeme::Ellipsis(EllipsisLexeme::new())
}
pub fn period() -> Lexeme {
    Lexeme::Period(PeriodLexeme::new())
}
pub fn newline() -> Lexeme {
    Lexeme::Newline(NewlineLexeme::new())
}

pub fn unlexable(txt: impl Into<String>) -> Lexeme {
    Lexeme::Unlexable(UnlexableLexeme::new(txt.into()))
}
