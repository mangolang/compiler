use mango::ast_full::AST;
use mango::token::Tokens;
use mango::util::encdec::ToText;

/// Represents an unparseable list of tokens.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnparseableAST {
    tokens: Vec<Box<Tokens>>,
}

impl UnparseableAST {
    pub fn from_tokens(tokens: Vec<Box<Tokens>>) -> UnparseableAST {
        UnparseableAST { tokens: tokens }
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
