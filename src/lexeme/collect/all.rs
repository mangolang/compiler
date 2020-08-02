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
use crate::io::slice::{SourceSlice, SourceLocation};

//TODO @mark: pass code slice along with lexeme

/// Collection of all possible lexemes.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Lexeme {
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

impl SourceLocation for Lexeme {
    fn source(&self) -> &SourceSlice {
        match self {
            Lexeme::Association(association) => association.source(),
            Lexeme::Identifier(identifier) => identifier.source(),
            Lexeme::Keyword(keyword) => keyword.source(),
            Lexeme::Literal(literal) => literal.source(),
            Lexeme::Operator(operator) => operator.source(),
            Lexeme::ParenthesisOpen(parenthesis_open) => parenthesis_open.source(),
            Lexeme::ParenthesisClose(parenthesis_close) => parenthesis_close.source(),
            Lexeme::BracketOpen(parenthesis_open) => parenthesis_open.source(),
            Lexeme::BracketClose(parenthesis_close) => parenthesis_close.source(),
            //Lexemes::EndStatement(end_statement) => end_statement.source(),
            Lexeme::StartBlock(start_block) => start_block.source(),
            Lexeme::EndBlock(end_block) => end_block.source(),
            Lexeme::Colon(colon) => colon.source(),
            Lexeme::Comma(comma) => comma.source(),
            Lexeme::Ellipsis(ellipsis) => ellipsis.source(),
            Lexeme::Period(period) => period.source(),
            Lexeme::Newline(newline) => newline.source(),
            Lexeme::Unlexable(unlexable) => unlexable.source(),
        }
    }
}

impl fmt::Debug for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexeme::Association(association) => write!(f, "as:{}", association.to_text()),
            Lexeme::Identifier(identifier) => write!(f, "${}", identifier.name),
            Lexeme::Keyword(keyword) => write!(f, "{}", keyword.word.to_str().as_ref().to_uppercase()),
            Lexeme::Literal(literal) => write!(f, "'{}'", literal.to_text()),
            Lexeme::Operator(operator) => write!(f, "op:{}", operator.to_text()),
            Lexeme::ParenthesisOpen(parenthesis_open) => write!(f, "'('"),
            Lexeme::ParenthesisClose(parenthesis_close) => write!(f, "')'"),
            Lexeme::BracketOpen(parenthesis_open) => write!(f, "'['"),
            Lexeme::BracketClose(parenthesis_close) => write!(f, "']'"),
            //Lexemes::EndStatement(end_statement) => write!(f, "end_statement"),
            Lexeme::StartBlock(start_block) => write!(f, "start_block"),
            Lexeme::EndBlock(end_block) => write!(f, "end_block"),
            Lexeme::Colon(colon) => write!(f, ":"),
            Lexeme::Comma(comma) => write!(f, "comma"),
            Lexeme::Ellipsis(ellipsis) => write!(f, "..."),
            Lexeme::Period(period) => write!(f, "."),
            Lexeme::Newline(newline) => writeln!(f, "NL"),
            Lexeme::Unlexable(unlexable) => write!(f, "??{}??", unlexable.text()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::lexeme::Lexeme;

    use super::*;

    const LONG_SIZE: usize = size_of::<f64>();

    #[test]
    fn test_lexemes_size() {
        assert!(size_of::<Lexeme>() <= 5 * LONG_SIZE, size_of::<Lexeme>());
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
