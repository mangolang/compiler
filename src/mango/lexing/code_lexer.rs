use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::special::UnlexableToken;
use mango::token::tokens::ParenthesisCloseToken;
use mango::token::tokens::ParenthesisOpenToken;
use mango::token::Tokens;
use mango::util::codeparts::Keyword;
use std::collections::VecDeque;

pub struct CodeLexer<'r> {
    reader: &'r mut Reader,
    indent: i32,
    // This is unfortunate, would not be needed with 'yield' but is now for indents
    buffer: VecDeque<Tokens>,
}

impl<'r> CodeLexer<'r> {
    fn new(reader: &'r mut Reader) -> Self {
        CodeLexer {
            reader,
            indent: 0,
            buffer: VecDeque::with_capacity(16),
        }
    }
}

impl<'r> Lexer<'r> for CodeLexer<'r> {
    fn lex(&mut self) -> MaybeToken {
        // If there is a buffer due to indentation or continuations, return from that.
        if !self.buffer.is_empty() {
            return MaybeToken::Token(self.buffer.pop_front().unwrap());
        }
        if let Match(word) = self.reader.matches("\\.\\.\\.") {
            // Line continuation has no token, it just continues on the next line.
            if let Match(word) = self.reader.matches("\\n\\r?") {
                // There should always be a newline after continuations, so that they can be ignored together.
            } else if let Match(word) = self.reader.matches("[^\\n]*\\n\\r?") {
                return MaybeToken::Token(Tokens::Unlexable(UnlexableToken::new(word)));
            } else {
                // TODO: I don't know yet how to deal with continuation followed by end of file
                panic!()
            }
        }
        // Indentation done; do the rest of lexing.
        if let Match(word) = self.reader.matches("(") {
            return MaybeToken::Token(Tokens::ParenthesisOpen(ParenthesisOpenToken::new()));
        }
        if let Match(word) = self.reader.matches(")") {
            return MaybeToken::Token(Tokens::ParenthesisClose(ParenthesisCloseToken::new()));
        }

        // TODO: a lot more

        // TODO: specify the unlexable word
        return MaybeToken::Token(Tokens::Unlexable(UnlexableToken::new("TODO".to_owned())));
    }
}
