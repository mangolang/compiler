use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::lexeme::collect::{association, operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;

/// Lex an arithmetic or boolean operator.
pub fn lex_operator(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(sym) = reader.strip_match(&*SYMBOL_RE) {
        lexer.add(operator(sym.as_str()).unwrap());
    }
}

/// Lex an equals sign, with optional arithmetic or boolean operator prefix.
pub fn lex_association(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(sym) = reader.strip_match(&*ASSOCIATION_RE) {
        lexer.add(association(sym.as_str()).unwrap());
    }
}

#[cfg(test)]
mod operators {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexing::lexer::token_collector::TokenCollector;
    use crate::lexeme::Tokens;
    use crate::lexeme::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;

    use super::lex_operator;

    fn check(input: &str, expected: &[Tokens]) {
        let expected: TokenCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_operator(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected);
    }

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn mismatch() {
        check("abc", &[]);
        check("abc+", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("abc ==", &[]);
    }

    #[test]
    fn plus() {
        check("+", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Plus))]);
    }

    #[test]
    fn dash() {
        check("-", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Dash))]);
    }

    #[test]
    fn asterisk() {
        check("*", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Asterisk))]);
    }

    #[test]
    fn slash() {
        check("/", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Slash))]);
    }

    #[test]
    fn lt() {
        check("<", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::LT))]);
    }

    #[test]
    fn gt() {
        check(">", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::GT))]);
    }

    #[test]
    fn eq() {
        check("==", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::EQ))]);
    }

    #[test]
    fn le() {
        check("<=", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::LE))]);
    }

    #[test]
    fn ge() {
        check(">=", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::GE))]);
    }

    #[test]
    fn exclamation() {
        check("!", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Exclamation))]);
    }

    #[test]
    fn question() {
        check("?", &[Tokens::Operator(OperatorToken::from_symbol(Symbol::Question))]);
    }

    #[test]
    fn all() {
        check(
            r"+-*/==<=>=<>!?",
            &[
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Plus)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Dash)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Asterisk)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Slash)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::EQ)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::LE)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::GE)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::LT)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::GT)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Exclamation)),
                Tokens::Operator(OperatorToken::from_symbol(Symbol::Question)),
            ],
        );
    }
}

#[cfg(test)]
mod associations {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{AssociationToken, Tokens};
    use crate::lexing::lexer::token_collector::TokenCollector;
    use crate::lexeme::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;

    use super::lex_association;

    fn check(input: &str, expected: &[Tokens]) {
        let expected: TokenCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_association(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected);
    }

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn mismatch() {
        check("abc", &[]);
        check("abc+", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("abc +=", &[]);
    }

    #[test]
    fn plain() {
        check("=", &[Tokens::Association(AssociationToken::from_unprefixed())]);
    }

    #[test]
    fn prefix() {
        check("+=", &[Tokens::Association(AssociationToken::from_symbol(Symbol::Plus).unwrap())]);
        check("-=", &[Tokens::Association(AssociationToken::from_symbol(Symbol::Dash).unwrap())]);
        check(
            "*=",
            &[Tokens::Association(AssociationToken::from_symbol(Symbol::Asterisk).unwrap())],
        );
        check("/=", &[Tokens::Association(AssociationToken::from_symbol(Symbol::Slash).unwrap())]);
        //check("!=", &[Tokens::Association(AssociationToken::from_symbol(Symbol::Exclamation).unwrap())]);
        //check("?=", &[Tokens::Association(AssociationToken::from_symbol(Symbol::Question).unwrap())]);
    }

    #[test]
    fn all() {
        check(
            r"+=-=*=/=",
            &[
                Tokens::Association(AssociationToken::from_symbol(Symbol::Plus).unwrap()),
                Tokens::Association(AssociationToken::from_symbol(Symbol::Dash).unwrap()),
                Tokens::Association(AssociationToken::from_symbol(Symbol::Asterisk).unwrap()),
                Tokens::Association(AssociationToken::from_symbol(Symbol::Slash).unwrap()),
            ],
        );
    }
}
