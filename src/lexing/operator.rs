use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::lexeme::collect::short::operator;
use crate::lexeme::collect::short::association;

/// Lex an arithmetic or boolean operator.
pub fn lex_operator(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(source) = reader.strip_match(&*SYMBOL_RE) {
        lexer.add(operator(source.as_str(), source.clone()).unwrap());
    }
}

/// Lex an equals sign, with optional arithmetic or boolean operator prefix.
pub fn lex_association(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(source) = reader.strip_match(&*ASSOCIATION_RE) {
        lexer.add(association(source.as_str(), source.clone()).unwrap());
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
    use crate::lexeme::collect::for_test::operator;

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
        check("+", &[operator(Symbol::Plus).into()]);
    }

    #[test]
    fn dash() {
        check("-", &[operator(Symbol::Dash).into()]);
    }

    #[test]
    fn asterisk() {
        check("*", &[operator(Symbol::Asterisk).into()]);
    }

    #[test]
    fn slash() {
        check("/", &[operator(Symbol::Slash).into()]);
    }

    #[test]
    fn lt() {
        check("<", &[operator(Symbol::LT).into()]);
    }

    #[test]
    fn gt() {
        check(">", &[operator(Symbol::GT).into()]);
    }

    #[test]
    fn eq() {
        check("==", &[operator(Symbol::EQ).into()]);
    }

    #[test]
    fn le() {
        check("<=", &[operator(Symbol::LE).into()]);
    }

    #[test]
    fn ge() {
        check(">=", &[operator(Symbol::GE).into()]);
    }

    #[test]
    fn exclamation() {
        check("!", &[operator(Symbol::Exclamation).into()]);
    }

    #[test]
    fn question() {
        check("?", &[operator(Symbol::Question).into()]);
    }

    #[test]
    fn all() {
        check(
            r"+-*/==<=>=<>!?",
            &[
                operator(Symbol::Plus).into(),
                operator(Symbol::Dash).into(),
                operator(Symbol::Asterisk).into(),
                operator(Symbol::Slash).into(),
                operator(Symbol::EQ).into(),
                operator(Symbol::LE).into(),
                operator(Symbol::GE).into(),
                operator(Symbol::LT).into(),
                operator(Symbol::GT).into(),
                operator(Symbol::Exclamation).into(),
                operator(Symbol::Question).into(),
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
    use crate::lexeme::collect::for_test::association;
    use crate::io::slice::SourceSlice;

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
        check("=", &[Lexeme::Association(AssociationLexeme::from_unprefixed(SourceSlice::mock()))]);
    }

    #[test]
    fn prefix() {
        check("+=", &[association(Symbol::Plus).into()]);
        check("-=", &[association(Symbol::Dash).into()]);
        check(
            "*=",
            &[association(Symbol::Asterisk).into()],
        );
        check("/=", &[association(Symbol::Slash).into()]);
        //check("!=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Exclamation).unwrap())]);
        //check("?=", &[Lexemes::Association(AssociationLexeme::from_symbol(Symbol::Question).unwrap())]);
    }

    #[test]
    fn all() {
        check(
            r"+=-=*=/=",
            &[
                association(Symbol::Plus).into(),
                association(Symbol::Dash).into(),
                association(Symbol::Asterisk).into(),
                association(Symbol::Slash).into(),
            ],
        );
    }
}
