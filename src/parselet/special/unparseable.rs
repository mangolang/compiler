use crate::parselet::Parselet;
use crate::lexeme::Lexemes;
use crate::util::encdec::ToText;

/// Represents an unparseable list of lexemes.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnparseableParselet {
    lexemes: Vec<Lexemes>,
}

impl UnparseableParselet {
    pub fn from_lexemes(lexemes: Vec<Lexemes>) -> UnparseableParselet {
        UnparseableParselet { lexemes }
    }
}

// impl ToText for UnparseableParselet {
//     fn to_text(&self) -> String {
//         format!(
//             " [cannot parse: {}] ",
//             self.lexemes.iter().map(Lexemes::to_text).collect::<Vec<String>>().join(" / ")
//         )
//     }
// }

impl Parselet for UnparseableParselet {}
