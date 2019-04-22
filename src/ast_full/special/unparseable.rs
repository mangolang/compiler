use crate::ast_full::AST;
use crate::token::Tokens;
use crate::util::encdec::ToText;

/// Represents an unparseable list of tokens.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnparseableAST {
    // should I box the tokens to save space? Probably not, they're not that big (40) and pointer is 8 overhead plus time
    tokens: Vec<Box<Tokens>>,
}

impl UnparseableAST {
    pub fn from_tokens(tokens: Vec<Box<Tokens>>) -> UnparseableAST {
        UnparseableAST { tokens }
    }
}

impl ToText for UnparseableAST {
    fn to_text(&self) -> String {
        format!(
            " [cannot parse: {}] ",
            self.tokens
                .iter()
                .map(|token: &Box<Tokens>| token.to_text())
                .collect::<Vec<String>>()
                .join(" / ")
        )
    }
}

impl AST for UnparseableAST {}
