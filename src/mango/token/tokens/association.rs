use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::strtype::Msg;
use mango::util::symbols::Symbol;

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AssociationToken {
    symbol: Option<Symbol>,
}

impl AssociationToken {
    pub fn from_str(symbol_txt: &str) -> Result<AssociationToken, Msg> {
        Ok(AssociationToken::from_symbol(Symbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: Symbol) -> AssociationToken {
        AssociationToken {
            symbol: Option::Some(symbol),
        }
    }
}

impl ToText for AssociationToken {
    fn to_text(&self) -> String {
        // LATER: this seems to compile, but IDEA flags it...
        match self.symbol {
            Option::None => " = ".to_owned(),
            Option::Some(ref sym) => format!(" {}= ", sym),
        }
    }
}

impl Token for AssociationToken {}
