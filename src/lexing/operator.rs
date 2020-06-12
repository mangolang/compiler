use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::operator::SYMBOL_RE;

/// Lex an arithmetic or boolean operator.
pub fn lex_operator(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    while let ReaderResult::Match(sym) = reader.strip_match(&*SYMBOL_RE) {
        lexer.add(operator(sym.as_str()).unwrap());
    }
}

#[cfg(test)]
mod operators {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::Tokens;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;

    use super::lex_operator;

    fn check(input: &str, expected: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_operator(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), expected);
    }

    #[test]
    fn empty() {
        check("", &vec![]);
    }

    #[test]
    fn mismatch() {
        check("abc", &vec![]);
        check("abc+", &vec![]);
    }

    #[test]
    fn plus() {
        check("+", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Plus))]);
    }
    #[test]
    fn dash() {
        check("-", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Dash))]);
    }
    #[test]
    fn asterisk() {
        check("*", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Asterisk))]);
    }
    #[test]
    fn slash() {
        check("/", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Slash))]);
    }
    #[test]
    fn lt() {
        check("<", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::LT))]);
    }
    #[test]
    fn gt() {
        check(">", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::GT))]);
    }
    #[test]
    fn eq() {
        check("==", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::EQ))]);
    }
    #[test]
    fn le() {
        check("<=", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::LE))]);
    }
    #[test]
    fn ge() {
        check(">=", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::GE))]);
    }
    #[test]
    fn exclamation() {
        check("!", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Exclamation))]);
    }
    #[test]
    fn question() {
        check("?", &vec![Tokens::Operator(OperatorToken::from_symbol(Symbol::Question))]);
    }
}
