use mango::token::Tokens;
use mango::util::encdec::to_text::ToText;

pub trait TokenStream {
    fn take(&self) -> Option<Tokens>;
    fn peek(&mut self) -> Option<&Tokens>;
}
