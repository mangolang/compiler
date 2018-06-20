use mango::io::typ::Reader;
use mango::lexing::code_lexer::CodeLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::lexing::typ::SubLexer;
use mango::lexing::typ::SubLexerResult;
use mango::token::Tokens;
use mango::util::collection::Queue;
use mango::util::collection::Stack;

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
            reader: reader,
            lexers: lexers,
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
                match lexer.lex_pass(&mut self.reader) {
                    SubLexerResult::Result(tokens) => {
                        if tokens.len() > 0 {
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
    use mango::io::fortest::StringReader;
    use mango::lexing::util::lex_all::{lex_all, LexList};
    use mango::token::tokens::AssociationToken;
    use mango::token::tokens::EndBlockToken;
    use mango::token::tokens::EndStatementToken;
    use mango::token::tokens::IdentifierToken;
    use mango::token::tokens::KeywordToken;
    use mango::token::tokens::LiteralToken;
    use mango::token::tokens::OperatorToken;
    use mango::token::tokens::StartBlockToken;
    use mango::token::Tokens;
    use mango::util::encdec::to_text::ToText;

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
                Tokens::Keyword(KeywordToken::from_str("let".to_owned()).unwrap()),
                Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap()),
                Tokens::Association(AssociationToken::from_unprefixed()),
                Tokens::Literal(LiteralToken::Int(0)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::Keyword(KeywordToken::from_str("for".to_owned()).unwrap()),
                Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap()),
                Tokens::Operator(OperatorToken::from_str("<").unwrap()),
                Tokens::Literal(LiteralToken::Int(128)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::StartBlock(StartBlockToken::new()),
                Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap()),
                Tokens::Association(AssociationToken::from_str("+".to_owned()).unwrap()),
                Tokens::Literal(LiteralToken::Int(1)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::EndBlock(EndBlockToken::new(true, false)),
            ],
        );
    }

    #[test]
    fn test_lexing_delegation() {}
}
