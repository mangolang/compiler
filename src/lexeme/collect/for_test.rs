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
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::parselet::file::import::ImportParselet;
use crate::lexeme::collect::FileLexemes;
use crate::io::source::SourceFile;
use crate::parsing::util::cursor::ParseCursor;

pub struct TestLexemeBuilder {
    source: String,
    /// End position of source slice, and lexeme
    lexemes: Vec<(usize, Box<dyn FnOnce(SourceSlice) -> Lexeme>)>,
}

pub fn builder() -> TestLexemeBuilder {
    TestLexemeBuilder {
        source: String::with_capacity(64),
        lexemes: Vec::with_capacity(8),
    }
}

impl TestLexemeBuilder {
    pub fn build(self) -> FileLexemes {
        let file = SourceFile::new("[mock]", self.source);
        let mut lexs = Vec::with_capacity(self.lexemes.len());
        let mut last = 0;
        for lex in self.lexemes {
            let slice = SourceSlice::new(&file, last, lex.0 - 1);
            last = lex.0;
            lexs.push(lex.1(slice))
        }
        FileLexemes::new(lexs)
    }

    /// Return end index of source slice.
    fn add_src(&mut self, txt: impl AsRef<str>) -> usize {
        self.source.push_str(txt.as_ref());
        self.source.push(' ');
        return self.source.len()
    }

    pub fn identifier(mut self, txt: impl Into<String>) -> Self {
        let txt = txt.into();
        let end = self.add_src(&txt);
        let lex = move |src| Lexeme::Identifier(IdentifierLexeme::from_str(&txt, src).unwrap());
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    fn add_keyword(mut self, kw: Keyword) -> Self {
        let end = self.add_src(kw.to_str());
        let lex = move |src| Lexeme::Keyword(KeywordLexeme::from_keyword(kw, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    /// Parse a keyword, including reserved keywords for future use.
    pub fn keyword_or_reserved(mut self, kw: impl IntoKeyword) -> Self {
        let kw = kw.keyword().unwrap();
        self.add_keyword(kw)
    }

    /// Parse a keyword, but fail if it is a reserved keyword, rather than one that already works.
    pub fn keyword(mut self, kw: impl IntoKeyword) -> Self {
        let kw = kw.keyword().unwrap();
        if let Keyword::Reserved(word) = kw {
            panic!("Keyword '{}' is reserved but not implemented", word);
        }
        self.add_keyword(kw)
    }

    pub fn literal_text(mut self, txt: impl Into<String>) -> Self {
        let txt = txt.into();
        let end = self.add_src(format!("\"{}\"", &txt));
        let lex = move |src| Lexeme::Literal(LiteralLexeme::Text(ustr(txt.as_ref()), src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn literal_int(mut self, nr: i64) -> Self {
        let end = self.add_src(format!("\"{}\"", nr));
        let lex = move |src| Lexeme::Literal(LiteralLexeme::Int(nr, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn literal_real(mut self, nr: impl Into<f64eq>) -> Self {
        let nr = nr.into();
        let end = self.add_src(format!("\"{}\"", nr));
        let lex = move |src| Lexeme::Literal(LiteralLexeme::Real(nr, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn literal_bool(mut self, truthy: bool) -> Self {
        let end = self.add_src(if truthy { "true" } else { "false" });
        let lex = move |src| Lexeme::Literal(LiteralLexeme::Boolean(truthy, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    // //TODO @mark:
    // pub fn operator(mut self, txt: impl IntoSymbol) {
    //     OperatorLexeme::from_symbol(txt.symbol(false).unwrap().unwrap(), SourceSlice::mock())
    // }
    //
    // //TODO @mark:
    // pub fn association(mut self, txt: impl IntoSymbol) {
    //     txt.symbol(true)
    //         .unwrap()
    //         .map(|sym| AssociationLexeme::from_symbol(sym, SourceSlice::mock()).unwrap())
    //         .unwrap_or_else(|| AssociationLexeme::from_unprefixed(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn parenthesis_open(mut self) {
    //     Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn parenthesis_close(mut self) {
    //     Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn bracket_open(mut self) {
    //     Lexeme::BracketOpen(BracketOpenLexeme::new(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn bracket_close(mut self) {
    //     Lexeme::BracketClose(BracketCloseLexeme::new(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn start_block(mut self) {
    //     Lexeme::StartBlock(StartBlockLexeme::new(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn end_block(mut self) {
    //     Lexeme::EndBlock(EndBlockLexeme::new2(SourceSlice::mock()))
    // }
    //
    // //TODO @mark:
    // pub fn colon(mut self) {
    //     Lexeme::Colon(ColonLexeme::new(SourceSlice::mock()))
    // }
    // //TODO @mark:
    // pub fn comma(mut self) {
    //     Lexeme::Comma(CommaLexeme::new(SourceSlice::mock()))
    // }
    // //TODO @mark:
    // pub fn ellipsis(mut self) {
    //     Lexeme::Ellipsis(EllipsisLexeme::new(SourceSlice::mock()))
    // }

    pub fn period(mut self) -> Self {
        let end = self.add_src(".");
        let lex = move |src| Lexeme::Period(PeriodLexeme::new(src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    // //TODO @mark:
    // pub fn slash(mut self) {
    //     Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Slash, SourceSlice::mock()))
    // }

    pub fn newline(mut self) -> Self {
        let end = self.add_src(";\n");
        let lex = move |src| Lexeme::Newline(NewlineLexeme::new(src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    // //TODO @mark:
    // pub fn unlexable(mut self, text: impl Into<String>) {
    //     Lexeme::Unlexable(UnlexableLexeme::new(text.into(), SourceSlice::mock()))
    // }
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
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
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn keyword_or_reserved(kw: impl IntoKeyword) -> Lexeme {
    let kw = kw.keyword().unwrap();
    Lexeme::Keyword(KeywordLexeme::from_keyword(kw, SourceSlice::mock()))
}

/// Parse a keyword, but fail if it is a reserved keyword, rather than one that already works.
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn keyword_supported(kw: impl IntoKeyword) -> Lexeme {
    let kw = kw.keyword().unwrap();
    if let Keyword::Reserved(word) = kw {
        panic!("Keyword '{}' is reserved but not implemented", word);
    }
    Lexeme::Keyword(KeywordLexeme::from_keyword(kw, SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn literal_text(txt: impl AsRef<str>) -> LiteralLexeme {
    LiteralLexeme::Text(ustr(txt.as_ref()), SourceSlice::mock())
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn literal_int(nr: i64) -> LiteralLexeme {
    LiteralLexeme::Int(nr, SourceSlice::mock())
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn literal_real(nr: impl Into<f64eq>) -> LiteralLexeme {
    LiteralLexeme::Real(nr.into(), SourceSlice::mock())
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
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

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn operator(txt: impl IntoSymbol) -> OperatorLexeme {
    OperatorLexeme::from_symbol(txt.symbol(false).unwrap().unwrap(), SourceSlice::mock())
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn association(txt: impl IntoSymbol) -> AssociationLexeme {
    txt.symbol(true)
        .unwrap()
        .map(|sym| AssociationLexeme::from_symbol(sym, SourceSlice::mock()).unwrap())
        .unwrap_or_else(|| AssociationLexeme::from_unprefixed(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn parenthesis_open() -> Lexeme {
    Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn parenthesis_close() -> Lexeme {
    Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn bracket_open() -> Lexeme {
    Lexeme::BracketOpen(BracketOpenLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn bracket_close() -> Lexeme {
    Lexeme::BracketClose(BracketCloseLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn start_block() -> Lexeme {
    Lexeme::StartBlock(StartBlockLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn end_block() -> Lexeme {
    Lexeme::EndBlock(EndBlockLexeme::new2(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn colon() -> Lexeme {
    Lexeme::Colon(ColonLexeme::new(SourceSlice::mock()))
}
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn comma() -> Lexeme {
    Lexeme::Comma(CommaLexeme::new(SourceSlice::mock()))
}
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn ellipsis() -> Lexeme {
    Lexeme::Ellipsis(EllipsisLexeme::new(SourceSlice::mock()))
}
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn period() -> Lexeme {
    Lexeme::Period(PeriodLexeme::new(SourceSlice::mock()))
}
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn slash() -> Lexeme {
    Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Slash, SourceSlice::mock()))
}
#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn newline() -> Lexeme {
    Lexeme::Newline(NewlineLexeme::new(SourceSlice::mock()))
}

#[deprecated(note="please use `TestLexemeBuilder` instead")]
pub fn unlexable(text: impl Into<String>) -> Lexeme {
    Lexeme::Unlexable(UnlexableLexeme::new(text.into(), SourceSlice::mock()))
}
