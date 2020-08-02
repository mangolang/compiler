use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::util::codeparts::Symbol;
use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AssociationLexeme {
    //TODO @mark: note that some symbols aren't allowed
    symbol: Option<Symbol>,
    source: SourceSlice,
}

impl AssociationLexeme {
    pub fn from_str(association_txt: &str, source: SourceSlice) -> MsgResult<AssociationLexeme> {
        if association_txt == "=" {
            return Ok(AssociationLexeme::from_unprefixed(source));
        }
        assert!(association_txt.ends_with('='));
        let symbol = Symbol::new(&association_txt[0..association_txt.len() - 1])?;
        AssociationLexeme::from_symbol(symbol, source)
    }

    pub fn from_unprefixed(source: SourceSlice) -> Self {
        AssociationLexeme {
            symbol: Option::None,
            source,
        }
    }

    pub fn from_symbol(symbol: Symbol, source: SourceSlice) -> MsgResult<Self> {
        let is_valid = match symbol {
            Symbol::Plus => true,
            Symbol::Dash => true,
            Symbol::Asterisk => true,
            Symbol::Slash => true,
            Symbol::Exclamation => unimplemented!(),
            Symbol::Question => unimplemented!(),
            // Binary boolean operators are not allowed before '='
            Symbol::LT => false,
            Symbol::GT => false,
            Symbol::EQ => false,
            Symbol::LE => false,
            Symbol::GE => false,
        };
        if !is_valid {
            return Err(ErrMsg::new(format!(
                "Symbol cannot be used as association (before an '='): '{}'",
                symbol
            )));
        }
        Ok(AssociationLexeme {
            symbol: Option::Some(symbol),
            source,
        })
    }
}

impl SourceLocation for AssociationLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for AssociationLexeme {
    fn to_text(&self) -> String {
        // LATER: this seems to compile, but IDEA flags it...
        match self.symbol {
            Option::None => "=".to_owned(),
            Option::Some(ref sym) => format!("{}=", sym),
        }
    }
}

#[cfg(test)]
mod from_str {
    use crate::common::tests::assert_panic_silent;
    use crate::lexeme::Lexeme;

    use super::*;

    #[test]
    fn empty() {
        assert_panic_silent(|| AssociationLexeme::from_str(""));
    }

    #[test]
    fn mismatch() {
        let err = AssociationLexeme::from_str("abc=").unwrap_err();
        assert!(err.as_str().to_lowercase().contains("unknown symbol"));
        assert!(err.as_str().to_lowercase().contains("abc"));
    }

    #[test]
    fn valid() {
        assert_eq!(
            AssociationLexeme::from_str("+=").unwrap(),
            AssociationLexeme::from_symbol(Symbol::Plus).unwrap()
        );
        assert_eq!(
            AssociationLexeme::from_str("-=").unwrap(),
            AssociationLexeme::from_symbol(Symbol::Dash).unwrap()
        );
        assert_eq!(
            AssociationLexeme::from_str("*=").unwrap(),
            AssociationLexeme::from_symbol(Symbol::Asterisk).unwrap()
        );
        assert_eq!(
            AssociationLexeme::from_str("/=").unwrap(),
            AssociationLexeme::from_symbol(Symbol::Slash).unwrap()
        );
        //assert_eq!(AssociationLexeme::from_str("?=").unwrap(), AssociationLexeme::from_symbol(Symbol::Exclamation));
        //assert_eq!(AssociationLexeme::from_str("!=").unwrap(), AssociationLexeme::from_symbol(Symbol::Question));
    }

    #[test]
    fn invalid() {
        assert!(AssociationLexeme::from_str("===").is_err());
        assert!(AssociationLexeme::from_str("<=").is_err());
        assert!(AssociationLexeme::from_str(">=").is_err());
        assert!(AssociationLexeme::from_str("<==").is_err());
        assert!(AssociationLexeme::from_str(">==").is_err());
    }
}
