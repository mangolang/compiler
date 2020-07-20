use crate::parselet::AST;
use crate::lexeme::Lexemes;
use crate::util::encdec::ToText;

/// Represents an unparseable list of lexemes.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnparseableAST {
    // should I box the lexemes to save space? Probably not, they're not that big (40) and pointer is 8 overhead plus time
    lexemes: Vec<Lexemes>,
}

impl UnparseableAST {
    pub fn from_lexemes(lexemes: Vec<Lexemes>) -> UnparseableAST {
        UnparseableAST { lexemes }
    }
}

// impl ToText for UnparseableAST {
//     fn to_text(&self) -> String {
//         format!(
//             " [cannot parse: {}] ",
//             self.lexemes.iter().map(Lexemes::to_text).collect::<Vec<String>>().join(" / ")
//         )
//     }
// }

impl AST for UnparseableAST {}
