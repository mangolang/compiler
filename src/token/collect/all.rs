use smallvec::SmallVec;

use crate::token::special::EndBlockToken;
use crate::token::special::StartBlockToken;
use crate::token::special::UnlexableToken;
use crate::token::tokens::AssociationToken;
use crate::token::tokens::EndStatementToken;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::KeywordToken;
use crate::token::tokens::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::token::tokens::ParenthesisCloseToken;
use crate::token::tokens::ParenthesisOpenToken;
use crate::util::encdec::ToText;
use crate::token::tokens::separators::{CommaToken, EllipsisToken, PeriodToken, NewlineToken};

/// Collection of all possible tokens.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Tokens {
    Association(AssociationToken),
    Identifier(IdentifierToken),
    Keyword(KeywordToken),
    Literal(LiteralToken),
    Operator(OperatorToken),
    ParenthesisOpen(ParenthesisOpenToken),
    ParenthesisClose(ParenthesisCloseToken),
    EndStatement(EndStatementToken),
    StartBlock(StartBlockToken),
    EndBlock(EndBlockToken),
    Comma(CommaToken),
    Ellipsis(EllipsisToken),
    Period(PeriodToken),
    Newline(NewlineToken),
    Unlexable(UnlexableToken),
}

//TODO @mark: optimize the bugger of TokenVec by benchmarking
pub type TokenVec = SmallVec<[Tokens; 2]>;

// impl ToText for Tokens {
//     fn to_text(&self) -> String {
//         use self::Tokens::*;
//         match self {
//             Association(token) => token.to_text(),
//             Identifier(token) => token.to_text(),
//             Keyword(token) => token.to_text(),
//             Literal(token) => token.to_text(),
//             Operator(token) => token.to_text(),
//             ParenthesisOpen(token) => token.to_text(),
//             ParenthesisClose(token) => token.to_text(),
//             EndStatement(token) => token.to_text(),
//             Unlexable(token) => token.to_text(),
//             StartBlock(token) => token.to_text(),
//             CommaToken(token) => token.to_text(),
//             EllipsisToken(token) => token.to_text(),
//             PeriodToken(token) => token.to_text(),
//             NewlineToken(token) => token.to_text(),
//             EndBlock(token) => token.to_text(),
//         }
//     }
// }

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
