
/// The different operator symbols that are recognized.
pub enum Symbol {
    Plus =     "+",
    Min =      "-",
    Asterisk = "*",
    Slash =    "/",
}

impl Symbol {
    pub fn new(symbol_txt: String) -> Result<Symbol, Msg> {
        return match symbol_txt {
            "+" => Ok(Symbol::Plus),
            "-" => Ok(Symbol::Plus),
            "*" => Ok(Symbol::Plus),
            "/" => Ok(Symbol::Plus),
            _ => Err(Msg::new(format!("Unknown symbol: '{}'", symbol_txt)))
        }
    }
}

/// An operator (unary, binary, ...).
pub struct Operator {
    symbol: Symbol

}

impl Operator {
    pub fn from_str(symbol_txt: String) -> Result<Operator, Msg> {
        return Operator::from_symbol(Symbol::new(symbol_txt)?);
    }

    pub fn from_symbol(symbol: Symbol) -> Operator {
        return Operator { symbol: symbol };
    }

    pub fn is_add_sub(&self) {
        return self.symbol == "+" || self.symbol == "-"
    }

    pub fn is_mul_div(&self) {
        return self.symbol == "*" || self.symbol == "/"
    }
}