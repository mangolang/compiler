use ::std::fmt;

use smallvec::SmallVec;

use crate::token::brackets::{BracketCloseToken, BracketOpenToken};
use crate::token::separators::ColonToken;
use crate::token::special::EndBlockToken;
use crate::token::special::StartBlockToken;
use crate::token::special::UnlexableToken;
use crate::token::tokens::separators::{CommaToken, EllipsisToken, NewlineToken, PeriodToken};
use crate::token::tokens::AssociationToken;
use crate::token::tokens::EndStatementToken;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::KeywordToken;
use crate::token::tokens::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::token::tokens::ParenthesisCloseToken;
use crate::token::tokens::ParenthesisOpenToken;
use crate::util::encdec::ToText;

/// Collection of all possible tokens.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Tokens {
    Association(AssociationToken),
    Identifier(IdentifierToken),
    Keyword(KeywordToken),
    Literal(LiteralToken),
    Operator(OperatorToken),
    ParenthesisOpen(ParenthesisOpenToken),
    ParenthesisClose(ParenthesisCloseToken),
    BracketOpen(BracketOpenToken),
    BracketClose(BracketCloseToken),
    // EndStatement(EndStatementToken),
    StartBlock(StartBlockToken),
    EndBlock(EndBlockToken),
    Colon(ColonToken),
    Comma(CommaToken),
    Ellipsis(EllipsisToken),
    Period(PeriodToken),
    Newline(NewlineToken),
    Unlexable(UnlexableToken),
}

impl fmt::Debug for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::Association(association) => write!(f, "as:{}", association.to_text()),
            Tokens::Identifier(identifier) => write!(f, "${}", identifier.name),
            Tokens::Keyword(keyword) => write!(f, "{}", keyword.word.to_str().as_ref().to_uppercase()),
            Tokens::Literal(literal) => write!(f, "'{}'", literal.to_text()),
            Tokens::Operator(operator) => write!(f, "op:{}", operator.to_text()),
            Tokens::ParenthesisOpen(parenthesis_open) => write!(f, "'('"),
            Tokens::ParenthesisClose(parenthesis_close) => write!(f, "')'"),
            Tokens::BracketOpen(parenthesis_open) => write!(f, "'['"),
            Tokens::BracketClose(parenthesis_close) => write!(f, "']'"),
            //Tokens::EndStatement(end_statement) => write!(f, "end_statement"),
            Tokens::StartBlock(start_block) => write!(f, "start_block"),
            Tokens::EndBlock(end_block) => write!(f, "end_block"),
            Tokens::Colon(colon) => write!(f, ":"),
            Tokens::Comma(comma) => write!(f, "comma"),
            Tokens::Ellipsis(ellipsis) => write!(f, "..."),
            Tokens::Period(period) => write!(f, "."),
            Tokens::Newline(newline) => writeln!(f, "NL"),
            Tokens::Unlexable(unlexable) => write!(f, "??{}??", unlexable.text),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::token::Tokens;

    use super::*;

    const LONG_SIZE: usize = size_of::<f64>();

    #[test]
    fn test_tokens_size() {
        assert!(size_of::<Tokens>() <= 5 * LONG_SIZE, size_of::<Tokens>());
    }

    //TODO @mark: these tests seem useless, they're already covered by `test_tokens_size` :

    #[test]
    fn test_association_token_size() {
        assert!(
            size_of::<AssociationToken>() <= 4 * LONG_SIZE,
            format!("AssociationToken size: {}", size_of::<AssociationToken>())
        );
    }

    #[test]
    fn test_identifier_token_size() {
        assert!(
            size_of::<IdentifierToken>() <= 4 * LONG_SIZE,
            format!("IdentifierToken size: {}", size_of::<IdentifierToken>())
        );
    }

    #[test]
    fn test_keyword_token_size() {
        assert!(
            size_of::<KeywordToken>() <= 4 * LONG_SIZE,
            format!("KeywordToken size: {}", size_of::<KeywordToken>())
        );
    }

    #[test]
    fn test_literal_token_size() {
        assert!(
            size_of::<LiteralToken>() <= 4 * LONG_SIZE,
            format!("LiteralToken size: {}", size_of::<LiteralToken>())
        );
    }

    #[test]
    fn test_operator_token_size() {
        assert!(
            size_of::<OperatorToken>() <= 4 * LONG_SIZE,
            format!("OperatorToken size: {}", size_of::<OperatorToken>())
        );
    }

    #[test]
    fn test_parenthesis_open_token_size() {
        assert!(
            size_of::<ParenthesisOpenToken>() <= 4 * LONG_SIZE,
            format!("ParenthesisOpenToken size: {}", size_of::<ParenthesisOpenToken>())
        );
    }

    #[test]
    fn test_parenthesis_close_token_size() {
        assert!(
            size_of::<ParenthesisCloseToken>() <= 4 * LONG_SIZE,
            format!("ParenthesisCloseToken size: {}", size_of::<ParenthesisCloseToken>())
        );
    }

    #[test]
    fn test_end_statement_token_size() {
        assert!(
            size_of::<EndStatementToken>() <= 4 * LONG_SIZE,
            format!("EndStatementToken size: {}", size_of::<EndStatementToken>())
        );
    }

    #[test]
    fn test_start_block_token_size() {
        assert!(
            size_of::<StartBlockToken>() <= 4 * LONG_SIZE,
            format!("StartBlockToken size: {}", size_of::<StartBlockToken>())
        );
    }

    #[test]
    fn test_end_block_token_size() {
        assert!(
            size_of::<EndBlockToken>() <= 4 * LONG_SIZE,
            format!("EndBlockToken size: {}", size_of::<EndBlockToken>())
        );
    }

    #[test]
    fn test_unlexable_token_size() {
        assert!(
            size_of::<UnlexableToken>() <= 4 * LONG_SIZE,
            format!("UnlexableToken size: {}", size_of::<UnlexableToken>())
        );
    }
}
