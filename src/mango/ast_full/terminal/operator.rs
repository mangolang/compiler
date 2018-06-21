use mango::ast_full::AST;
use mango::util::codeparts::Symbol;
use mango::util::encdec::ToText;
use mango::util::strtype::Msg;

/// An operator (unary, binary, ...).
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OperatorAST {
    symbol: Symbol,
}

impl OperatorAST {
    pub fn from_str(symbol_txt: &str) -> Result<OperatorAST, Msg> {
        Ok(OperatorAST::from_symbol(Symbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: Symbol) -> OperatorAST {
        OperatorAST { symbol: symbol }
    }

    pub fn is_add_sub(&self) -> bool {
        self.symbol == Symbol::Plus || self.symbol == Symbol::Dash
    }

    pub fn is_mul_div(&self) -> bool {
        self.symbol == Symbol::Asterisk || self.symbol == Symbol::Slash
    }
}

impl ToText for OperatorAST {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}

impl AST for OperatorAST {}
