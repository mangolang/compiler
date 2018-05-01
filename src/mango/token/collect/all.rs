use mango::token::special::UnlexableToken;
use mango::util::encdec::ToText;

/// Collection of all possible tokens.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Tokens {
    //    Operator(OperatorAST),
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
