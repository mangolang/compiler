
mod reader;
mod lexer;
mod lex;
// mod old_lexer;
// mod util;
mod literals;
mod identifier;
mod operator;
mod indent;
mod grouping;
mod special;
mod separators;

#[cfg(test)]
mod tests;

pub use self::lex::lex;
