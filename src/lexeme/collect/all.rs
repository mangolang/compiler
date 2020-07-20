use ::std::fmt;

use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::special::EndBlockLexeme;
use crate::lexeme::special::StartBlockLexeme;
use crate::lexeme::special::UnlexableLexeme;
use crate::lexeme::lexemes::AssociationLexeme;
use crate::lexeme::lexemes::EndStatementLexeme;
use crate::lexeme::lexemes::IdentifierLexeme;
use crate::lexeme::lexemes::KeywordLexeme;
use crate::lexeme::lexemes::LiteralLexeme;
use crate::lexeme::lexemes::OperatorLexeme;
use crate::lexeme::lexemes::ParenthesisCloseLexeme;
use crate::lexeme::lexemes::ParenthesisOpenLexeme;
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::util::encdec::ToText;

/// Collection of all possible lexemes.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Lexemes {
    Association(AssociationLexeme),
    Identifier(IdentifierLexeme),
    Keyword(KeywordLexeme),
    Literal(LiteralLexeme),
    Operator(OperatorLexeme),
    ParenthesisOpen(ParenthesisOpenLexeme),
    ParenthesisClose(ParenthesisCloseLexeme),
    BracketOpen(BracketOpenLexeme),
    BracketClose(BracketCloseLexeme),
    // EndStatement(EndStatementLexeme),
    StartBlock(StartBlockLexeme),
    EndBlock(EndBlockLexeme),
    Colon(ColonLexeme),
    Comma(CommaLexeme),
    Ellipsis(EllipsisLexeme),
    Period(PeriodLexeme),
    Newline(NewlineLexeme),
    Unlexable(UnlexableLexeme),
}

impl fmt::Debug for Lexemes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexemes::Association(association) => write!(f, "as:{}", association.to_text()),
            Lexemes::Identifier(identifier) => write!(f, "${}", identifier.name),
            Lexemes::Keyword(keyword) => write!(f, "{}", keyword.word.to_str().as_ref().to_uppercase()),
            Lexemes::Literal(literal) => write!(f, "'{}'", literal.to_text()),
            Lexemes::Operator(operator) => write!(f, "op:{}", operator.to_text()),
            Lexemes::ParenthesisOpen(parenthesis_open) => write!(f, "'('"),
            Lexemes::ParenthesisClose(parenthesis_close) => write!(f, "')'"),
            Lexemes::BracketOpen(parenthesis_open) => write!(f, "'['"),
            Lexemes::BracketClose(parenthesis_close) => write!(f, "']'"),
            //Lexemes::EndStatement(end_statement) => write!(f, "end_statement"),
            Lexemes::StartBlock(start_block) => write!(f, "start_block"),
            Lexemes::EndBlock(end_block) => write!(f, "end_block"),
            Lexemes::Colon(colon) => write!(f, ":"),
            Lexemes::Comma(comma) => write!(f, "comma"),
            Lexemes::Ellipsis(ellipsis) => write!(f, "..."),
            Lexemes::Period(period) => write!(f, "."),
            Lexemes::Newline(newline) => writeln!(f, "NL"),
            Lexemes::Unlexable(unlexable) => write!(f, "??{}??", unlexable.text),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::lexeme::Lexemes;

    use super::*;

    const LONG_SIZE: usize = size_of::<f64>();

    #[test]
    fn test_lexemes_size() {
        assert!(size_of::<Lexemes>() <= 5 * LONG_SIZE, size_of::<Lexemes>());
    }

    //TODO @mark: these tests seem useless, they're already covered by `test_lexemes_size` :

    #[test]
    fn test_association_lexeme_size() {
        assert!(
            size_of::<AssociationLexeme>() <= 4 * LONG_SIZE,
            format!("AssociationLexeme size: {}", size_of::<AssociationLexeme>())
        );
    }

    #[test]
    fn test_identifier_lexeme_size() {
        assert!(
            size_of::<IdentifierLexeme>() <= 4 * LONG_SIZE,
            format!("IdentifierLexeme size: {}", size_of::<IdentifierLexeme>())
        );
    }

    #[test]
    fn test_keyword_lexeme_size() {
        assert!(
            size_of::<KeywordLexeme>() <= 4 * LONG_SIZE,
            format!("KeywordLexeme size: {}", size_of::<KeywordLexeme>())
        );
    }

    #[test]
    fn test_literal_lexeme_size() {
        assert!(
            size_of::<LiteralLexeme>() <= 4 * LONG_SIZE,
            format!("LiteralLexeme size: {}", size_of::<LiteralLexeme>())
        );
    }

    #[test]
    fn test_operator_lexeme_size() {
        assert!(
            size_of::<OperatorLexeme>() <= 4 * LONG_SIZE,
            format!("OperatorLexeme size: {}", size_of::<OperatorLexeme>())
        );
    }

    #[test]
    fn test_parenthesis_open_lexeme_size() {
        assert!(
            size_of::<ParenthesisOpenLexeme>() <= 4 * LONG_SIZE,
            format!("ParenthesisOpenLexeme size: {}", size_of::<ParenthesisOpenLexeme>())
        );
    }

    #[test]
    fn test_parenthesis_close_lexeme_size() {
        assert!(
            size_of::<ParenthesisCloseLexeme>() <= 4 * LONG_SIZE,
            format!("ParenthesisCloseLexeme size: {}", size_of::<ParenthesisCloseLexeme>())
        );
    }

    #[test]
    fn test_end_statement_lexeme_size() {
        assert!(
            size_of::<EndStatementLexeme>() <= 4 * LONG_SIZE,
            format!("EndStatementLexeme size: {}", size_of::<EndStatementLexeme>())
        );
    }

    #[test]
    fn test_start_block_lexeme_size() {
        assert!(
            size_of::<StartBlockLexeme>() <= 4 * LONG_SIZE,
            format!("StartBlockLexeme size: {}", size_of::<StartBlockLexeme>())
        );
    }

    #[test]
    fn test_end_block_lexeme_size() {
        assert!(
            size_of::<EndBlockLexeme>() <= 4 * LONG_SIZE,
            format!("EndBlockLexeme size: {}", size_of::<EndBlockLexeme>())
        );
    }

    #[test]
    fn test_unlexable_lexeme_size() {
        assert!(
            size_of::<UnlexableLexeme>() <= 4 * LONG_SIZE,
            format!("UnlexableLexeme size: {}", size_of::<UnlexableLexeme>())
        );
    }
}
