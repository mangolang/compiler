use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexeme};
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
    use crate::lexeme::Lexeme;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;

    use super::lex_operator;

    fn check(input: &str, expected: &[Lexeme]) {
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
        check("+", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Plus))]);
    }

    #[test]
    fn dash() {
        check("-", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Dash))]);
    }

    #[test]
    fn asterisk() {
        check("*", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Asterisk))]);
    }

    #[test]
    fn slash() {
        check("/", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Slash))]);
    }

    #[test]
    fn lt() {
        check("<", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::LT))]);
    }

    #[test]
    fn gt() {
        check(">", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::GT))]);
    }

    #[test]
    fn eq() {
        check("==", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::EQ))]);
    }

    #[test]
    fn le() {
        check("<=", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::LE))]);
    }

    #[test]
    fn ge() {
        check(">=", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::GE))]);
    }

    #[test]
    fn exclamation() {
        check("!", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Exclamation))]);
    }

    #[test]
    fn question() {
        check("?", &[Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Question))]);
    }

    #[test]
    fn all() {
        check(
            r"+-*/==<=>=<>!?",
            &[
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Plus)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Dash)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Asterisk)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Slash)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::EQ)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::LE)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::GE)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::LT)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::GT)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Exclamation)),
                Lexeme::Operator(OperatorLexeme::from_symbol(Symbol::Question)),
            ],
        );
    }
}

#[cfg(test)]
mod associations {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{AssociationLexeme, Lexeme};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;

    use super::lex_association;

    fn check(input: &str, expected: &[Lexeme]) {
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
        check("=", &[Lexeme::Association(AssociationLexeme::from_unprefixed())]);
    }

    #[test]
    fn prefix() {
        check("+=", &[Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Plus).unwrap())]);
        check("-=", &[Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Dash).unwrap())]);
        check(
            "*=",
            &[Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Asterisk).unwrap())],
        );
        check("/=", &[Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Slash).unwrap())]);
        //check("!=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Exclamation).unwrap())]);
        //check("?=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Question).unwrap())]);
    }

    #[test]
    fn all() {
        check(
            r"+=-=*=/=",
            &[
                Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Plus).unwrap()),
                Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Dash).unwrap()),
                Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Asterisk).unwrap()),
                Lexeme::Association(AssociationLexeme::from_symbol(Symbol::Slash).unwrap()),
            ],
        );
    }
}
