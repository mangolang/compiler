use crate::token::special::UnlexableToken;
use crate::token::tokens::AssociationToken;
use crate::token::tokens::EndBlockToken;
use crate::token::tokens::EndStatementToken;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::KeywordToken;
use crate::token::tokens::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::token::tokens::ParenthesisCloseToken;
use crate::token::tokens::ParenthesisOpenToken;
use crate::token::tokens::StartBlockToken;
use crate::util::encdec::ToText;
use smallvec::SmallVec;

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
    Unlexable(UnlexableToken),
}

//TODO @mark: optimize the bugger of TokenVec by benchmarking
pub type TokenVec = SmallVec<[Tokens; 2]>;

impl ToText for Tokens {
    fn to_text(&self) -> String {
        use self::Tokens::*;
        match self {
            Association(token) => token.to_text(),
            Identifier(token) => token.to_text(),
            Keyword(token) => token.to_text(),
            Literal(token) => token.to_text(),
            Operator(token) => token.to_text(),
            ParenthesisOpen(token) => token.to_text(),
            ParenthesisClose(token) => token.to_text(),
            EndStatement(token) => token.to_text(),
            Unlexable(token) => token.to_text(),
            StartBlock(token) => token.to_text(),
            EndBlock(token) => token.to_text(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Tokens;
    use std::mem::size_of;
    use super::*;

    const LONG_SIZE: usize = size_of::<f64>();

    #[test]
    fn test_tokens_size() {
        assert!(size_of::<Tokens>() <= 4 * LONG_SIZE, size_of::<Tokens>());
    }

    #[test]
    fn test_association_token_size() {
        assert!(size_of::<AssociationToken>() <= 3 * LONG_SIZE, format!("AssociationToken size: {}", size_of::<AssociationToken>()));
    }

    #[test]
    fn test_identifier_token_size() {
        assert!(size_of::<IdentifierToken>() <= 3 * LONG_SIZE, format!("IdentifierToken size: {}", size_of::<IdentifierToken>()));
    }

    #[test]
    fn test_keyword_token_size() {
        assert!(size_of::<KeywordToken>() <= 3 * LONG_SIZE, format!("KeywordToken size: {}", size_of::<KeywordToken>()));
    }

    #[test]
    fn test_literal_token_size() {
        assert!(size_of::<LiteralToken>() <= 3 * LONG_SIZE, format!("LiteralToken size: {}", size_of::<LiteralToken>()));
    }

    #[test]
    fn test_operator_token_size() {
        assert!(size_of::<OperatorToken>() <= 3 * LONG_SIZE, format!("OperatorToken size: {}", size_of::<OperatorToken>()));
    }

    #[test]
    fn test_parenthesis_open_token_size() {
        assert!(size_of::<ParenthesisOpenToken>() <= 3 * LONG_SIZE, format!("ParenthesisOpenToken size: {}", size_of::<ParenthesisOpenToken>()));
    }

    #[test]
    fn test_parenthesis_close_token_size() {
        assert!(size_of::<ParenthesisCloseToken>() <= 3 * LONG_SIZE, format!("ParenthesisCloseToken size: {}", size_of::<ParenthesisCloseToken>()));
    }

    #[test]
    fn test_end_statement_token_size() {
        assert!(size_of::<EndStatementToken>() <= 3 * LONG_SIZE, format!("EndStatementToken size: {}", size_of::<EndStatementToken>()));
    }

    #[test]
    fn test_start_block_token_size() {
        assert!(size_of::<StartBlockToken>() <= 3 * LONG_SIZE, format!("StartBlockToken size: {}", size_of::<StartBlockToken>()));
    }

    #[test]
    fn test_end_block_token_size() {
        assert!(size_of::<EndBlockToken>() <= 3 * LONG_SIZE, format!("EndBlockToken size: {}", size_of::<EndBlockToken>()));
    }

    #[test]
    fn test_unlexable_token_size() {
        assert!(size_of::<UnlexableToken>() <= 3 * LONG_SIZE, format!("UnlexableToken size: {}", size_of::<UnlexableToken>()));
    }
}
