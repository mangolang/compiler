use mango::token::Tokens;
use mango::util::encdec::to_text::ToText;
use mango::util::signaltype::stream::StreamElem;

/// A stream of lexed [Tokens].
pub trait TokenStream {
    /// Take the next token from the stream.
    /// This advances the stream index, and can be called once per token. The [Tokens] may or may not be in the stream anymore after.
    /// [StreamToken::End] is returned after the last element, and will continue to be returned on further tries.
    fn take(&self) -> StreamElem<Tokens>;
    /// Looks at the current head of the stream, but does not advance the stream.
    fn peek(&mut self) -> StreamElem<&Tokens>;
}

#[derive(Debug)]
/// A [TokenStream] that simply iterates over an in-memory list. Intended for testing.
pub struct MemoryTokenStream {
    index: usize,
    tokens: Vec<Tokens>,
}

impl MemoryTokenStream {
    pub fn new(tokens: Vec<Tokens>) -> MemoryTokenStream {
        MemoryTokenStream { index: 0, tokens }
    }

    pub fn to_text(&self) -> String {
        format!(
            "{}",
            self.tokens
                .iter()
                .map(|token| token.to_text())
                .fold(String::new(), |s, a| s + &a)
        )
    }
}

impl TokenStream for MemoryTokenStream {
    fn take(&self) -> StreamElem<Tokens> {
        // later: It seemed easier and possibly faster to copy objects than to move them out of the vector
        if self.index >= self.tokens.len() {
            return StreamElem::End;
        }
        StreamElem::Elem(self.tokens[self.index].clone())
    }

    fn peek(&mut self) -> StreamElem<&Tokens> {
        if self.index >= self.tokens.len() {
            return StreamElem::End;
        }
        self.index += 1;
        StreamElem::Elem(&self.tokens[self.index])
    }
}
