use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::symbols::Symbol;
use mango::util::strtype::Msg;

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AssociationSymbolToken {
    symbol: Option<Symbol>,
}

impl AssociationSymbolToken {
    pub fn from_str(symbol_txt: &str) -> Result<AssociationSymbolToken, Msg> {
        Ok(AssociationSymbolToken::from_symbol(Symbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: Symbol) -> AssociationSymbolToken {
        AssociationSymbolToken { symbol: Option::Some(symbol) }
    }
}

impl ToText for AssociationSymbolToken {
    fn to_text(&self) -> String {
        match self.symbol {
            Option::None => " = ".to_owned(),
            Option::Some(ref sym) => format!(" {}= ", sym),
        }
    }
}

impl Token for AssociationSymbolToken {}

