use mango::util::encdec::ToText;
use mango::token::special::UnlexableToken;

/// Collection of all possible tokens.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Tokens {
//    Operator(OperatorAST),

    Unlexable(UnlexableToken),  // todo: spelling
}

impl ToText for Tokens {
    fn to_text(&self) -> String {
        match self {
//            Tokens::Operator(operator) => operator.to_text(),
            _ => unimplemented!(), // TODO
        }
    }
}
