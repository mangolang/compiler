use mango::token::special::UnlexableToken;
use mango::token::tokens::AssociationToken;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::KeywordToken;
use mango::token::tokens::LiteralToken;
use mango::token::tokens::OperatorToken;
use mango::token::tokens::ParenthesisCloseToken;
use mango::token::tokens::ParenthesisOpenToken;
use mango::util::encdec::ToText;

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
    Unlexable(UnlexableToken), // todo: spelling
                               // TODO
}

impl ToText for Tokens {
    fn to_text(&self) -> String {
        match self {
            //            Tokens::Operator(operator) => operator.to_text(),
            _ => unimplemented!(), // TODO
        }
    }
}
