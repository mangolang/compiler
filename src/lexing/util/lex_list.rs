use crate::lexeme::collect::all::Lexemes;
use crate::util::encdec::to_text::ToText;

/// Represents all the lex lexemes in a source.
#[derive(PartialEq, Eq, Debug)]
pub struct LexList {
    lexemes: Vec<Lexemes>,
}

impl LexList {
    pub fn from_lexemes(lexemes: Vec<Lexemes>) -> Self {
        LexList { lexemes }
    }
}

impl ToText for LexList {
    fn to_text(&self) -> String {
        self.lexemes.iter().map(ToText::to_text).collect::<Vec<_>>().join(" ")
    }
}
