use crate::token::Token;
use crate::util::codeparts::Symbol;
use crate::util::encdec::ToText;
use crate::util::strtype::Msg;

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AssociationToken {
    symbol: Option<Symbol>,
}

impl AssociationToken {
    pub fn from_unprefixed() -> Self {
        AssociationToken { symbol: Option::None }
    }

    pub fn from_str<S: Into<String>>(symbol_txt: S) -> Result<AssociationToken, Msg> {
        Ok(AssociationToken::from_symbol(Symbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: Symbol) -> Self {
        AssociationToken {
            symbol: Option::Some(symbol),
        }
    }

    pub fn subpattern() -> String {
        format!(r"(?:{})?=", Symbol::subpattern())
    }
}

impl ToText for AssociationToken {
    fn to_text(&self) -> String {
        // LATER: this seems to compile, but IDEA flags it...
        match self.symbol {
            Option::None => "=".to_owned(),
            Option::Some(ref sym) => format!("{}=", sym),
        }
    }
}

impl Token for AssociationToken {}
