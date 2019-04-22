use crate::io::typ::Reader;
use crate::lexing::code_lexer::CodeLexer;
use crate::lexing::typ::Lexer;
use crate::lexing::typ::MaybeToken;
use crate::lexing::typ::SubLexer;
use crate::lexing::typ::SubLexerResult;
use crate::token::Tokens;
use crate::util::collection::Queue;
use crate::util::collection::Stack;

pub struct CombiLexer {
    reader: Box<Reader>,
    lexers: Stack<Box<SubLexer>>,
    buffer: Queue<Tokens>,
}

impl CombiLexer {
    pub fn new(reader: Box<Reader>) -> Self {
        let mut lexers: Stack<Box<SubLexer>> = Stack::new();
        lexers.push(Box::new(CodeLexer::new()));
        CombiLexer {
            reader,
            lexers,
            buffer: Queue::new(),
        }
    }
}

impl Lexer for CombiLexer {
    fn lex(&mut self) -> MaybeToken {
        // If there are tokens in the buffer, return from there;
        if let Option::Some(token) = self.buffer.pop() {
            return MaybeToken::Token(token);
        }

        match self.lexers.borrow_mut() {
            // No more lexers to delegate to; lexing is finished.
            Option::None => MaybeToken::End,
            Option::Some(ref mut lexer) => {
                match lexer.lex_pass(&mut *self.reader) {
                    SubLexerResult::Result(tokens) => {
                        if !tokens.is_empty() {
                            // The sublexer produced tokens, queue them.
                            self.buffer.append(tokens);
                            self.lex() // TODO: if every branch does this, move it down
                        } else {
                            // No tokens were produced; make sure the reader has advanced to prevent infinite loops.
                            // TODO: check reader state
                            self.lex()
                        }
                    }
                    SubLexerResult::Delegate(lexer) => {
                        // Switch to a different delegate lexer.
                        self.lexers.push(lexer);
                        self.lex()
                    }
                    SubLexerResult::End => {
                        // The sublexer is done, remove it from the stack and continue with the next.
                        self.lexers.pop(); // This needs non-lexical lifetimes
                        self.lex()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CombiLexer;
    use crate::io::fortest::StringReader;
    use crate::lexing::util::lex_all::{lex_all, LexList};
    use crate::token::tokens::AssociationToken;
    use crate::token::tokens::EndBlockToken;
    use crate::token::tokens::EndStatementToken;
    use crate::token::tokens::IdentifierToken;
    use crate::token::tokens::KeywordToken;
    use crate::token::tokens::LiteralToken;
    use crate::token::tokens::OperatorToken;
    use crate::token::tokens::StartBlockToken;
    use crate::token::Tokens;
    use crate::util::encdec::to_text::ToText;
    use std::str::FromStr;

    fn assert_text_to_tokens(text: &str, tokens: Vec<Tokens>) {
        let expected = LexList::from_tokens(tokens);
        let actual = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(text.to_owned()))));
        assert_eq!(
            expected,
            actual,
            "\nexpected:\n{}\nactual:\n{}",
            expected.to_text(),
            actual.to_text(),
        );
    }

    #[test]
    fn test_lexing_combined() {
        assert_text_to_tokens(
            "let x = 0\nfor x < 128\n\tx += 1",
            vec![
                Tokens::Keyword(KeywordToken::from_str("let").unwrap()),
                Tokens::Identifier(IdentifierToken::from_str("x").unwrap()),
                Tokens::Association(AssociationToken::from_unprefixed()),
                Tokens::Literal(LiteralToken::Int(0)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::Keyword(KeywordToken::from_str("for").unwrap()),
                Tokens::Identifier(IdentifierToken::from_str("x").unwrap()),
                Tokens::Operator(OperatorToken::from_str("<").unwrap()),
                Tokens::Literal(LiteralToken::Int(128)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::StartBlock(StartBlockToken::new()),
                Tokens::Identifier(IdentifierToken::from_str("x").unwrap()),
                Tokens::Association(AssociationToken::from_str("+").unwrap()),
                Tokens::Literal(LiteralToken::Int(1)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::EndBlock(EndBlockToken::new(true, false)),
            ],
        );
    }

    #[test]
    fn test_lexing_delegation() {}
}
