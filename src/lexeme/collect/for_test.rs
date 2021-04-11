use ::std::fmt;
use ::std::str::FromStr;

use ::ustr::ustr;

use crate::common::codeparts::{Keyword, Symbol};
use crate::common::codeparts::eqfloat::f64eq;
use crate::dbg_log;
use crate::io::slice::SourceSlice;
use crate::io::source::SourceFile;
use crate::lexeme::{
    AssociationLexeme, EndBlockLexeme, FQIdentifierLexeme, KeywordLexeme, Lexeme, LiteralLexeme, OperatorLexeme, ParenthesisCloseLexeme,
    ParenthesisOpenLexeme, StartBlockLexeme, UnlexableLexeme,
};
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::collect::FileLexemes;
use crate::lexeme::collect::print::print_lexeme;
#[cfg(test)]
use crate::lexeme::collect::print::print_lexemes;
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::literal::TextLiteralLexeme;

pub type LexemeGenerator = Box<dyn FnOnce(SourceSlice) -> Lexeme>;

pub struct TestLexemeBuilder {
    source: String,
    /// End position of source slice, and lexeme
    lexemes: Vec<(usize, LexemeGenerator)>,
}

pub fn builder() -> TestLexemeBuilder {
    TestLexemeBuilder {
        source: String::with_capacity(64),
        lexemes: Vec::with_capacity(8),
    }
}

pub trait IntoKeyword: fmt::Debug {
    fn keyword(self) -> Keyword;
}

impl IntoKeyword for &str {
    fn keyword(self) -> Keyword {
        Keyword::from_str(self).unwrap()
    }
}

impl IntoKeyword for Keyword {
    fn keyword(self) -> Keyword {
        self
    }
}

pub trait IntoSymbol: fmt::Debug {
    fn symbol(self, is_association: bool) -> Option<Symbol>;
}

impl IntoSymbol for &str {
    /// Get the symbol. If this is for an association, strip a character (should be "=") from the end of the text.
    fn symbol(self, is_association: bool) -> Option<Symbol> {
        let symbol_txt = if is_association { &self[0..self.len() - 1] } else { self };
        if symbol_txt.is_empty() {
            return None;
        }
        Some(Symbol::new(symbol_txt).unwrap())
    }
}

impl IntoSymbol for Symbol {
    fn symbol(self, _is_association: bool) -> Option<Symbol> {
        Some(self)
    }
}

impl TestLexemeBuilder {
    pub fn build(self) -> Vec<Lexeme> {
        let file = SourceFile::new("[mock]", self.source);
        let mut lexs = Vec::with_capacity(self.lexemes.len());
        let mut prev = 0;
        for lex in self.lexemes {
            let slice = SourceSlice::new(&file, prev, lex.0 - 1);
            prev = lex.0;
            lexs.push(lex.1(slice))
        }
        //TODO @mark: better formatting
        dbg_log!("<lexeme-input>\n{}\n</lexeme-input>", print_lexemes(&lexs));
        lexs
    }

    pub fn file(self) -> FileLexemes {
        FileLexemes::new(self.build())
    }

    pub fn build_single(self) -> Lexeme {
        assert_eq!(self.lexemes.len(), 1);
        let TestLexemeBuilder { source, mut lexemes } = self;
        let file = SourceFile::new("[mock]", source);
        let lex = lexemes.drain(..).next().unwrap();
        let slice = SourceSlice::new(&file, 0, lex.0 - 1);
        lex.1(slice)
    }

    /// Return end index of source slice.
    fn add_src(&mut self, txt: impl AsRef<str>) -> usize {
        self.source.push_str(txt.as_ref());
        self.source.push(' ');
        self.source.len()
    }

    fn add_simple(mut self, txt: impl AsRef<str>, lexeme_gen: fn(SourceSlice) -> Lexeme) -> Self {
        let end = self.add_src(&txt);
        self.lexemes.push((end, Box::new(lexeme_gen)));
        self
    }

    pub fn raw(mut self, additions: impl IntoIterator<Item=Lexeme>) -> Self {
        for lexeme in additions.into_iter() {
            let end = self.add_src(print_lexeme(&lexeme));
            self.lexemes.push((end, Box::new(|_| lexeme)));
        }
        self
    }

    pub fn identifier(mut self, txt: impl Into<String>) -> Self {
        let txt = txt.into();
        let end = self.add_src(&txt);
        let lex = move |src| Lexeme::Identifier(FQIdentifierLexeme::from_str(&txt, src).unwrap());
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
    pub fn keyword_or_reserved(self, kw: impl IntoKeyword) -> Self {
        let kw = kw.keyword();
        self.add_keyword(kw)
    }

    /// Parse a keyword, but fail if it is a reserved keyword, rather than one that already works.
    pub fn keyword(self, kw: impl IntoKeyword) -> Self {
        let kw = kw.keyword();
        if let Keyword::Reserved(word) = kw {
            panic!("Keyword '{}' is reserved but not implemented", word);
        }
        self.add_keyword(kw)
    }

    pub fn literal_text(mut self, txt: impl Into<String>) -> Self {
        let txt = txt.into();
        let end = self.add_src(format!("\"{}\"", &txt));
        let lex = move |src| Lexeme::Literal(LiteralLexeme::new_text(ustr(txt.as_ref()), src));
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

    pub fn operator(mut self, txt: impl IntoSymbol) -> Self {
        let sym = txt.symbol(false).unwrap();
        let end = self.add_src(format!("{}", sym));
        let lex = move |src| Lexeme::Operator(OperatorLexeme::from_symbol(sym, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn assignment(self) -> Self {
        self.add_simple("=", |src| Lexeme::Association(AssociationLexeme::from_unprefixed(src)))
    }

    pub fn association(mut self, txt: impl IntoSymbol) -> Self {
        let sym = txt.symbol(true);
        let end = self.add_src(if let Some(s) = &sym { format!("{}=", s) } else { "".to_owned() });
        let lex = |src: SourceSlice| {
            Lexeme::Association(match sym {
                Some(s) => AssociationLexeme::from_symbol(s, src).unwrap(),
                None => AssociationLexeme::from_unprefixed(src),
            })
        };
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn parenthesis_open(self) -> Self {
        self.add_simple("(", |src| Lexeme::ParenthesisOpen(ParenthesisOpenLexeme::new(src)))
    }

    pub fn parenthesis_close(self) -> Self {
        self.add_simple(")", |src| Lexeme::ParenthesisClose(ParenthesisCloseLexeme::new(src)))
    }

    pub fn bracket_open(self) -> Self {
        self.add_simple("[", |src| Lexeme::BracketOpen(BracketOpenLexeme::new(src)))
    }

    pub fn bracket_close(self) -> Self {
        self.add_simple("]", |src| Lexeme::BracketClose(BracketCloseLexeme::new(src)))
    }

    pub fn start_block(self) -> Self {
        self.add_simple("{\n", |src| Lexeme::StartBlock(StartBlockLexeme::new(src)))
    }

    pub fn end_block(self) -> Self {
        self.add_simple("}\n", |src| Lexeme::EndBlock(EndBlockLexeme::new2(src)))
    }

    pub fn colon(self) -> Self {
        self.add_simple(": ", |src| Lexeme::Colon(ColonLexeme::new(src)))
    }

    pub fn comma(self) -> Self {
        self.add_simple(",", |src| Lexeme::Comma(CommaLexeme::new(src)))
    }

    pub fn ellipsis(self) -> Self {
        self.add_simple("...", |src| Lexeme::Ellipsis(EllipsisLexeme::new(src)))
    }

    pub fn period(mut self) -> Self {
        let end = self.add_src(".");
        let lex = move |src| Lexeme::Period(PeriodLexeme::new(src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn newline(mut self) -> Self {
        let end = self.add_src(";\n");
        let lex = move |src| Lexeme::Newline(NewlineLexeme::new(src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }

    pub fn unlexable(mut self, txt: impl Into<String>) -> Self {
        let txt = txt.into();
        let end = self.add_src(";\n");
        let lex = move |src| Lexeme::Unlexable(UnlexableLexeme::new(txt, src));
        self.lexemes.push((end, Box::new(lex)));
        self
    }
}

pub fn identifier(txt: &str) -> FQIdentifierLexeme {
    FQIdentifierLexeme::from_str(&txt, SourceSlice::mock()).unwrap()
}

pub fn literal_text(txt: impl AsRef<str>) -> LiteralLexeme {
    LiteralLexeme::Text(TextLiteralLexeme::new(ustr(txt.as_ref()), SourceSlice::mock()))
}

pub fn literal_int(nr: i64) -> LiteralLexeme {
    LiteralLexeme::Int(nr, SourceSlice::mock())
}

pub fn literal_real(nr: impl Into<f64eq>) -> LiteralLexeme {
    LiteralLexeme::Real(nr.into(), SourceSlice::mock())
}

pub fn literal_bool(truthy: bool) -> LiteralLexeme {
    LiteralLexeme::Boolean(truthy, SourceSlice::mock())
}

pub fn operator(txt: impl IntoSymbol) -> OperatorLexeme {
    OperatorLexeme::from_symbol(txt.symbol(false).unwrap(), SourceSlice::mock())
}

pub fn association(txt: impl IntoSymbol) -> AssociationLexeme {
    txt.symbol(true)
        .map(|sym| AssociationLexeme::from_symbol(sym, SourceSlice::mock()).unwrap())
        .unwrap_or_else(|| AssociationLexeme::from_unprefixed(SourceSlice::mock()))
}
