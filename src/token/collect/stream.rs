use crate::token::Tokens;
use crate::util::encdec::to_text::ToText;
use crate::util::signaltype::stream::StreamElem;

/// A stream of lexed [Tokens].
pub trait TokenStream {
    /// Take the next token from the stream.
    /// This advances the stream index, and can be called once per token. The [Tokens] may or may not be in the stream anymore after.
    /// [StreamToken::End] is returned after the last element, and will continue to be returned on further tries.
    fn take(&self) -> StreamElem<Tokens>;
    /// Looks at the current head of the stream, but does not advance the stream.
    fn peek(&mut self) -> StreamElem<&Tokens>;
}

/// A [TokenStream] that simply iterates over an in-memory list. Intended for testing.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MemoryTokenStream {
    index: usize,
    tokens: Vec<Tokens>,
}

impl MemoryTokenStream {
    #[allow(dead_code)] // TODO: for now
    pub fn new(tokens: Vec<Tokens>) -> MemoryTokenStream {
        MemoryTokenStream { index: 0, tokens }
    }

    #[allow(dead_code)] // TODO: for now
    pub fn to_text(&self) -> String {
        format!(" {}", self.tokens.iter().map(Tokens::to_text).collect::<Vec<_>>().join(" "))
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
