use ::std::str::FromStr;

use ::ustr::ustr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::io::slice::SourceSlice;
use crate::ir::codeparts::eqfloat::f64eq;
use crate::ir::codeparts::Keyword;
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::{
    AssociationLexeme, EndBlockLexeme, FQIdentifierLexeme, KeywordLexeme, Lexeme, LiteralLexeme, OperatorLexeme, ParenthesisCloseLexeme,
    ParenthesisOpenLexeme, StartBlockLexeme, UnlexableLexeme,
};

//TODO @mark: replace more lexeme usages by short versions

pub fn association(txt: &str, source: SourceSlice) -> MsgResult<Lexeme> {
    Ok(Lexeme::Association(AssociationLexeme::from_str(txt, source)?))
}

pub fn identifier(txt: &str, source: SourceSlice) -> MsgResult<Lexeme> {
    Ok(Lexeme::Identifier(FQIdentifierLexeme::from_str(txt, source)?))
}

/// Parse a keyword, including reserved keywords for future use.
pub fn keyword_or_reserved(txt: &str, source: SourceSlice) -> MsgResult<Lexeme> {
    Ok(Lexeme::Keyword(KeywordLexeme::from_str(txt, source)?))
}

/// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
pub fn keyword_supported(txt: &str, source: SourceSlice) -> MsgResult<Lexeme> {
    match Keyword::from_str(txt)? {
        Keyword::Reserved(word) => Err(ErrMsg::new(format!("Keyword '{}' not implemented", word))),
        kw => Ok(Lexeme::Keyword(KeywordLexeme::from_keyword(kw, source))),
    }
}

pub fn literal_text(txt: impl AsRef<str>, source: SourceSlice) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::new_text(ustr(txt.as_ref()), source))
}

pub fn literal_int(nr: i64, source: SourceSlice) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Int(nr, source))
}

pub fn literal_real(nr: impl Into<f64eq>, source: SourceSlice) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Real(nr.into(), source))
}

pub fn literal_bool(b: bool, source: SourceSlice) -> Lexeme {
    Lexeme::Literal(LiteralLexeme::Boolean(b, source))
}

pub fn operator(txt: &str, source: SourceSlice) -> MsgResult<Lexeme> {
    Ok(Lexeme::Operator(OperatorLexeme::from_str(&txt, source)?))
}

pub fn parenthesis_open(source: SourceSlice) -> Lexeme {
    Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new(source))
}

pub fn parenthesis_close(source: SourceSlice) -> Lexeme {
    Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new(source))
}

pub fn bracket_open(source: SourceSlice) -> Lexeme {
    Lexeme::BracketOpen(BracketOpenLexeme::new(source))
}

pub fn bracket_close(source: SourceSlice) -> Lexeme {
    Lexeme::BracketClose(BracketCloseLexeme::new(source))
}

// pub fn end_statement() -> Lexemes {
//     //TODO @mark: for now only create newlines
//     Lexemes::EndStatement(EndStatementLexeme::new_end_line())
// }

pub fn start_block(source: SourceSlice) -> Lexeme {
    Lexeme::StartBlock(StartBlockLexeme::new(source))
}

pub fn end_block(source: SourceSlice) -> Lexeme {
    Lexeme::EndBlock(EndBlockLexeme::new2(source))
}

pub fn colon(source: SourceSlice) -> Lexeme {
    Lexeme::Colon(ColonLexeme::new(source))
}
pub fn comma(source: SourceSlice) -> Lexeme {
    Lexeme::Comma(CommaLexeme::new(source))
}
pub fn ellipsis(source: SourceSlice) -> Lexeme {
    Lexeme::Ellipsis(EllipsisLexeme::new(source))
}
pub fn period(source: SourceSlice) -> Lexeme {
    Lexeme::Period(PeriodLexeme::new(source))
}
pub fn newline(source: SourceSlice) -> Lexeme {
    Lexeme::Newline(NewlineLexeme::new(source))
}

pub fn unlexable(source: SourceSlice) -> Lexeme {
    Lexeme::Unlexable(UnlexableLexeme::from_source(source))
}
