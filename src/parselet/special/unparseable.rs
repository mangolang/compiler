use crate::parselet::AST;
use crate::lexeme::Tokens;
use crate::util::encdec::ToText;

/// Represents an unparseable list of tokens.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnparseableAST {
    // should I box the tokens to save space? Probably not, they're not that big (40) and pointer is 8 overhead plus time
    tokens: Vec<Tokens>,
}

impl UnparseableAST {
    pub fn from_tokens(tokens: Vec<Tokens>) -> UnparseableAST {
        UnparseableAST { tokens }
    }
}

// impl ToText for UnparseableAST {
//     fn to_text(&self) -> String {
//         format!(
//             " [cannot parse: {}] ",
//             self.tokens.iter().map(Tokens::to_text).collect::<Vec<String>>().join(" / ")
//         )
//     }
// }

impl AST for UnparseableAST {}
