use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexemes};
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
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::Lexemes;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;

    use super::lex_operator;

    fn check(input: &str, expected: &[Lexemes]) {
        let expected: LexemeCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_operator(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
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
        check("+", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Plus))]);
    }

    #[test]
    fn dash() {
        check("-", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Dash))]);
    }

    #[test]
    fn asterisk() {
        check("*", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Asterisk))]);
    }

    #[test]
    fn slash() {
        check("/", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Slash))]);
    }

    #[test]
    fn lt() {
        check("<", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::LT))]);
    }

    #[test]
    fn gt() {
        check(">", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::GT))]);
    }

    #[test]
    fn eq() {
        check("==", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::EQ))]);
    }

    #[test]
    fn le() {
        check("<=", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::LE))]);
    }

    #[test]
    fn ge() {
        check(">=", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::GE))]);
    }

    #[test]
    fn exclamation() {
        check("!", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Exclamation))]);
    }

    #[test]
    fn question() {
        check("?", &[Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Question))]);
    }

    #[test]
    fn all() {
        check(
            r"+-*/==<=>=<>!?",
            &[
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Plus)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Dash)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Asterisk)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Slash)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::EQ)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::LE)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::GE)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::LT)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::GT)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Exclamation)),
                Lexemes::Operator(OperatorLexeme::from_symbol(Symbol::Question)),
            ],
        );
    }
}

#[cfg(test)]
mod associations {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{AssociationLexeme, Lexemes};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;

    use super::lex_association;

    fn check(input: &str, expected: &[Lexemes]) {
        let expected: LexemeCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_association(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
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
        check("=", &[Lexemes::Association(AssociationLexeme::from_unprefixed())]);
    }

    #[test]
    fn prefix() {
        check("+=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Plus).unwrap())]);
        check("-=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Dash).unwrap())]);
        check(
            "*=",
            &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Asterisk).unwrap())],
        );
        check("/=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Slash).unwrap())]);
        //check("!=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Exclamation).unwrap())]);
        //check("?=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Question).unwrap())]);
    }

    #[test]
    fn all() {
        check(
            r"+=-=*=/=",
            &[
                Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Plus).unwrap()),
                Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Dash).unwrap()),
                Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Asterisk).unwrap()),
                Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Slash).unwrap()),
            ],
        );
    }
}
