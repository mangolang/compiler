use ::std::fmt;
use ::std::str::FromStr;

use ::ustr::ustr;

use crate::common::codeparts::{Keyword, Symbol};
use crate::common::codeparts::eqfloat::f64eq;
use crate::io::slice::SourceSlice;
use crate::lexeme::{
    AssociationLexeme, EndBlockLexeme, IdentifierLexeme, KeywordLexeme, Lexeme, LiteralLexeme, OperatorLexeme, ParenthesisCloseLexeme,
    ParenthesisOpenLexeme, StartBlockLexeme, UnlexableLexeme,
};
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::parselet::file::import::ImportParselet;

pub fn identifier(txt: &str) -> IdentifierLexeme {
    IdentifierLexeme::from_str(txt, SourceSlice::mock()).unwrap()
}

pub trait IntoKeyword: fmt::Debug {
    fn keyword(self) -> Result<Keyword, ()>;
}

impl IntoKeyword for &str {
    fn keyword(self) -> Result<Keyword, ()> {
        match Keyword::from_str(self) {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }
    }
}

impl IntoKeyword for Keyword {
    fn keyword(self) -> Result<Keyword, ()> {
        Ok(self)
    }
}

/// Parse a keyword, including reserved keywords for future use.
pub fn keyword_or_reserved(kw: impl IntoKeyword) -> Lexeme {
    let kw = kw.keyword().unwrap();
    Lexeme::Keyword(KeywordLexeme::from_keyword(kw, SourceSlice::mock()))
}

/// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
pub fn keyword_supported(kw: impl IntoKeyword) -> Lexeme {
    let kw = kw.keyword().unwrap();
    if let Keyword::Reserved(word) = kw {
        panic!("Keyword '{}' is reserved but not implemented", word);
    }
    Lexeme::Keyword(KeywordLexeme::from_keyword(kw, SourceSlice::mock()))
}

pub fn literal_text(txt: impl AsRef<str>) -> LiteralLexeme {
    LiteralLexeme::Text(ustr(txt.as_ref()), SourceSlice::mock())
}

pub fn literal_int(nr: i64) -> LiteralLexeme {
    LiteralLexeme::Int(nr, SourceSlice::mock())
}

pub fn literal_real(nr: impl Into<f64eq>) -> LiteralLexeme {
    LiteralLexeme::Real(nr.into(), SourceSlice::mock())
}

pub fn literal_bool(b: bool) -> LiteralLexeme {
    LiteralLexeme::Boolean(b, SourceSlice::mock())
}

pub trait IntoSymbol: fmt::Debug {
    fn symbol(self, is_association: bool) -> Result<Option<Symbol>, ()>;
}

impl IntoSymbol for &str {
    /// Get the symbol. If this is for an association, strip a character (should be "=") from the end of the text.
    fn symbol(self, is_association: bool) -> Result<Option<Symbol>, ()> {
        let symbol_txt = if is_association { &self[0..self.len() - 1] } else { self };
        if symbol_txt.is_empty() {
            return Ok(None);
        }
        match Symbol::new(symbol_txt) {
            Ok(s) => Ok(Some(s)),
            Err(_) => Err(()),
        }
    }
}

impl IntoSymbol for Symbol {
    fn symbol(self, _is_association: bool) -> Result<Option<Symbol>, ()> {
        Ok(Some(self))
    }
}

pub fn operator(txt: impl IntoSymbol) -> OperatorLexeme {
    OperatorLexeme::from_symbol(txt.symbol(false).unwrap().unwrap(), SourceSlice::mock())
}

pub fn association(txt: impl IntoSymbol) -> AssociationLexeme {
    txt.symbol(true)
        .unwrap()
        .map(|sym| AssociationLexeme::from_symbol(sym, SourceSlice::mock()).unwrap())
        .unwrap_or_else(|| AssociationLexeme::from_unprefixed(SourceSlice::mock()))
}

pub fn parenthesis_open() -> Lexeme {
    Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new(SourceSlice::mock()))
}

pub fn parenthesis_close() -> Lexeme {
    Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new(SourceSlice::mock()))
}

pub fn bracket_open() -> Lexeme {
    Lexeme::BracketOpen(BracketOpenLexeme::new(SourceSlice::mock()))
}

pub fn bracket_close() -> Lexeme {
    Lexeme::BracketClose(BracketCloseLexeme::new(SourceSlice::mock()))
}

pub fn start_block() -> Lexeme {
    Lexeme::StartBlock(StartBlockLexeme::new(SourceSlice::mock()))
}

pub fn end_block() -> Lexeme {
    Lexeme::EndBlock(EndBlockLexeme::new2(SourceSlice::mock()))
}

pub fn colon() -> Lexeme {
    Lexeme::Colon(ColonLexeme::new(SourceSlice::mock()))
}
pub fn comma() -> Lexeme {
    Lexeme::Comma(CommaLexeme::new(SourceSlice::mock()))
}
pub fn ellipsis() -> Lexeme {
    Lexeme::Ellipsis(EllipsisLexeme::new(SourceSlice::mock()))
}
pub fn period() -> Lexeme {
    Lexeme::Period(PeriodLexeme::new(SourceSlice::mock()))
}
pub fn slash() -> Lexeme {
    Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Slash, SourceSlice::mock()))
}
pub fn newline() -> Lexeme {
    Lexeme::Newline(NewlineLexeme::new(SourceSlice::mock()))
}

pub fn import(fqn: &str) -> ImportParselet {
    //TODO @mark: more general return type?
    let identifier = IdentifierLexeme::from_str(fqn, SourceSlice::mock()).unwrap();
    ImportParselet::new(identifier, None)
}

pub fn unlexable(text: impl Into<String>) -> Lexeme {
    Lexeme::Unlexable(UnlexableLexeme::new(text.into(), SourceSlice::mock()))
}
