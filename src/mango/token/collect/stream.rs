use mango::token::Tokens;
use mango::util::encdec::to_text::ToText;

pub trait TokenStream {
    fn take(&self) -> Option<Tokens>;
    fn peek(&mut self) -> Option<&Tokens>;
}

#[derive(Debug)]
pub struct MemoryTokenStream {
    index: usize,
    tokens: Vec<Tokens>
}

impl MemoryTokenStream {
    pub fn new(tokens: Vec<Tokens>) -> MemoryTokenStream {
        MemoryTokenStream { index: 0, tokens }
    }

    pub fn to_text(&self) -> String {
        format!("{}", self.tokens.iter().map(|token| token.to_text()).fold(String::new(), |s, a| s + &a))
    }
}

impl TokenStream for MemoryTokenStream {
    fn take(&self) -> Option<Tokens> {
        // later: It seemed easier and possibly faster to copy objects than to move them out of the vector
        if self.index >= self.tokens.len() {
            return Option::None
        }
        Option::Some(self.tokens[self.index].clone())
    }

    fn peek(&mut self) -> Option<&Tokens> {
        if self.index >= self.tokens.len() {
            return Option::None
        }
        self.index += 1;
        Option::Some(&self.tokens[self.index])
    }
}
