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

    #[test]
    fn test_tokens_size() {
        assert!(size_of::<Tokens>() <= 16, size_of::<Tokens>());
    }
}
