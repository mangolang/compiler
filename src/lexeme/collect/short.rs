use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::lexeme::{
    AssociationLexeme, EndBlockLexeme, EndStatementLexeme, IdentifierLexeme, KeywordLexeme, LiteralLexeme, OperatorLexeme, ParenthesisCloseLexeme,
    ParenthesisOpenLexeme, StartBlockLexeme, Lexemes, UnlexableLexeme,
};
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::util::codeparts::Keyword;
use crate::util::numtype::f64eq;

//TODO @mark: replace more lexeme usages by short versions

pub fn association(txt: &str) -> MsgResult<Lexemes> {
    Ok(Lexemes::Association(AssociationLexeme::from_str(txt)?))
}

pub fn identifier(txt: &str) -> MsgResult<Lexemes> {
    Ok(Lexemes::Identifier(IdentifierLexeme::from_str(txt)?))
}

/// Parse a keyword, including reserved keywords for future use.
pub fn keyword_or_reserved(txt: &str) -> MsgResult<Lexemes> {
    Ok(Lexemes::Keyword(KeywordLexeme::from_str(txt)?))
}

/// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
pub fn keyword_supported(txt: &str) -> MsgResult<Lexemes> {
    match Keyword::from_str(txt)? {
        Keyword::Reserved(word) => Err(ErrMsg::new(format!("Keyword '{}' not implemented", word))),
        kw => Ok(Lexemes::Keyword(KeywordLexeme::from_keyword(kw))),
    }
}

pub fn literal_text(txt: impl Into<String>) -> Lexemes {
    Lexemes::Literal(LiteralLexeme::Text(txt.into()))
}

pub fn literal_int(nr: i64) -> Lexemes {
    Lexemes::Literal(LiteralLexeme::Int(nr))
}

pub fn literal_real(nr: impl Into<f64eq>) -> Lexemes {
    Lexemes::Literal(LiteralLexeme::Real(nr.into()))
}

pub fn literal_bool(b: bool) -> Lexemes {
    Lexemes::Literal(LiteralLexeme::Boolean(b))
}

pub fn operator(txt: &str) -> MsgResult<Lexemes> {
    Ok(Lexemes::Operator(OperatorLexeme::from_str(&txt)?))
}

pub fn parenthesis_open() -> Lexemes {
    Lexemes::ParenthesisOpen(ParenthesisOpenLexeme::new())
}

pub fn parenthesis_close() -> Lexemes {
    Lexemes::ParenthesisClose(ParenthesisCloseLexeme::new())
}

pub fn bracket_open() -> Lexemes {
    Lexemes::BracketOpen(BracketOpenLexeme::new())
}

pub fn bracket_close() -> Lexemes {
    Lexemes::BracketClose(BracketCloseLexeme::new())
}

// pub fn end_statement() -> Lexemes {
//     //TODO @mark: for now only create newlines
//     Lexemes::EndStatement(EndStatementLexeme::new_end_line())
// }

pub fn start_block() -> Lexemes {
    Lexemes::StartBlock(StartBlockLexeme::new())
}

pub fn end_block() -> Lexemes {
    Lexemes::EndBlock(EndBlockLexeme::new2())
}

pub fn colon() -> Lexemes {
    Lexemes::Colon(ColonLexeme::new())
}
pub fn comma() -> Lexemes {
    Lexemes::Comma(CommaLexeme::new())
}
pub fn ellipsis() -> Lexemes {
    Lexemes::Ellipsis(EllipsisLexeme::new())
}
pub fn period() -> Lexemes {
    Lexemes::Period(PeriodLexeme::new())
}
pub fn newline() -> Lexemes {
    Lexemes::Newline(NewlineLexeme::new())
}

pub fn unlexable(txt: impl Into<String>) -> Lexemes {
    Lexemes::Unlexable(UnlexableLexeme::new(txt.into()))
}
