use ::std::fmt;

use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::brackets::{BracketCloseLexeme, BracketOpenLexeme};
use crate::lexeme::lexemes::separators::{CommaLexeme, EllipsisLexeme, NewlineLexeme, PeriodLexeme};
use crate::lexeme::lexemes::AssociationLexeme;
use crate::lexeme::lexemes::IdentifierLexeme;
use crate::lexeme::lexemes::KeywordLexeme;
use crate::lexeme::lexemes::LiteralLexeme;
use crate::lexeme::lexemes::OperatorLexeme;
use crate::lexeme::lexemes::ParenthesisCloseLexeme;
use crate::lexeme::lexemes::ParenthesisOpenLexeme;
use crate::lexeme::separators::ColonLexeme;
use crate::lexeme::special::EndBlockLexeme;
use crate::lexeme::special::StartBlockLexeme;
use crate::lexeme::special::UnlexableLexeme;
use crate::common::debug::ToText;
use crate::lexeme::EndStatementLexeme;

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
    //EndStatement(EndStatementLexeme),
    StartBlock(StartBlockLexeme),
    EndBlock(EndBlockLexeme),
    Colon(ColonLexeme),
    Comma(CommaLexeme),
    Ellipsis(EllipsisLexeme),
    Period(PeriodLexeme),
    Newline(NewlineLexeme),
    Unlexable(UnlexableLexeme),
}

impl Lexeme {
    // Note: if one day there are many is_* and as_* methods, find or write a macro.
    pub fn is_newline(&self) -> bool {
        matches!(self, Lexeme::Newline(_))
    }
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
            //Lexeme::EndStatement(end_statement) => end_statement.source(),
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
            Lexeme::ParenthesisOpen(_parenthesis_open) => write!(f, "'('"),
            Lexeme::ParenthesisClose(_parenthesis_close) => write!(f, "')'"),
            Lexeme::BracketOpen(_parenthesis_open) => write!(f, "'['"),
            Lexeme::BracketClose(_parenthesis_close) => write!(f, "']'"),
            //Lexeme::EndStatement(end_statement) => write!(f, ";"),
            Lexeme::StartBlock(_start_block) => write!(f, "start_block"),
            Lexeme::EndBlock(_end_block) => write!(f, "end_block"),
            Lexeme::Colon(_colon) => write!(f, ":"),
            Lexeme::Comma(_comma) => write!(f, "comma"),
            Lexeme::Ellipsis(_ellipsis) => write!(f, "..."),
            Lexeme::Period(_period) => write!(f, "."),
            Lexeme::Newline(_newline) => writeln!(f, "NL"),
            Lexeme::Unlexable(unlexable) => write!(f, "??{}??", unlexable.text()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::lexeme::Lexeme;

    const LONG_SIZE: usize = size_of::<f64>();

    #[test]
    fn test_lexemes_size() {
        //TODO @mark: is this limit too high?
        assert!(size_of::<Lexeme>() <= 8 * LONG_SIZE, "{}", size_of::<Lexeme>());
    }
}
