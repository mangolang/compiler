
// TODO: dead code, no longer used

use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::string_lexer::StringLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::special::UnlexableToken;
use mango::token::tokens::AssociationToken;
use mango::token::tokens::EndBlockToken;
use mango::token::tokens::EndStatementToken;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::KeywordToken;
use mango::token::tokens::OperatorToken;
use mango::token::tokens::ParenthesisCloseToken;
use mango::token::tokens::ParenthesisOpenToken;
use mango::token::tokens::StartBlockToken;
use mango::token::Tokens;
use mango::util::collection::Queue;
use std::cell::RefCell;
use std::rc::Rc;

// TODO: Preferably there'd be only one Lexer at a time which has a Reader, but I did not get that to work,
// TODO: see this SO question: https://stackoverflow.com/questions/50535022/borrow-checker-problems-for-parser-that-can-delegate

enum ReaderOrDelegate {
    Reader(),
    Delegate(Box<Lexer>),
}

pub struct CodeLexer {
    //    reader: Rc<RefCell<Reader>>,
    indent: i32,

    reader: Rc<RefCell<Reader>>,
    // This delegate deals with nested structures, like string literals and comments.
    reader_or_delegate: ReaderOrDelegate,
    // This is unfortunate, would not be needed with 'yield' but is now for indents.
    buffer: Queue<Tokens>,
}

impl CodeLexer {
    pub fn new(reader: Rc<RefCell<Reader>>) -> Self {
        CodeLexer {
            reader: reader,
            reader_or_delegate: ReaderOrDelegate::Reader(),
            indent: 0,
            buffer: Queue::new(),
        }
    }

    fn lex_indents(&mut self) -> MaybeToken {
        let mut line_indent = 0;
        while let Match(_) = self.reader.borrow_mut().matches("\\t") {
            line_indent += 1;
        }
        for _ in line_indent..self.indent {
            // This line is dedented, make end tokens.
            // TODO: turn this "new" into a constant
            if let Match(_) = self.reader.borrow_mut().matches("end") {
                // If this is followed by an 'end' keyword, then that 'end' is redundant.
                self.buffer
                    .push(Tokens::EndBlock(EndBlockToken::new(true, true)));
            } else {
                self.buffer
                    .push(Tokens::EndBlock(EndBlockToken::new(true, false)));
            }
        }
        for _ in self.indent..line_indent {
            // This line is indented, make start tokens.
            self.buffer.push(Tokens::StartBlock(StartBlockToken::new()));
        }
        self.indent = line_indent;
        self.lex()
    }
}

impl Lexer for CodeLexer {
    // TODO: TURN THIS AROUND: MAKE A FUNCTION THAT RETURNS FROM A QUEUE, AND CALLS ANOTHER TO FILL THE QUEUE IF NO RETURN

    fn lex(&mut self) -> MaybeToken {
        use self::MaybeToken::*;

        // If currently delegating to a sub-lexer, return from that.
        match self.reader_or_delegate {
            ReaderOrDelegate::Delegate(ref mut delegate) => {
                let delegated_token = delegate.lex();
                match delegated_token {
                    End => {
                        // Swap back from delegation to direct mode.
                        //                        let reader = delegate.get_reader().clone();
                        self.reader_or_delegate = ReaderOrDelegate::Reader();
                        self.lex()
                    }
                    Token(token) => Token(token),
                }
                // Code to stop delegation cannot be here, because `self` is still mutably borrowed through `delegate`
            }
            ReaderOrDelegate::Reader() => {
                // todo: maybe this branch could be a separate function?

                // If there is a buffer due to indentation or continuations, return from that.
                if let Some(token) = self.buffer.pop() {
                    return Token(token);
                }
                // Past this point, we assume that hte buffer is empty. When adding stuff, pop it or re-enter lex() soon.
                let continue_match_res = self.reader.borrow_mut().matches("\\.\\.\\.");
                if let Match(_) = continue_match_res {
                    // Line continuation has no token, it just continues on the next line.
                    let newline_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                    if let Match(_) = newline_match_res {
                        // There should always be a newline after continuations, so that they can be ignored together.
                    } else {
                        let newline_match_res = self.reader.borrow_mut().matches("[^\\n]*\\n\\r?");
                        if let Match(word) = newline_match_res {
                            self.buffer
                                .push(Tokens::Unlexable(UnlexableToken::new(word)));
                            // This is a new line, so there may be indents.
                            self.lex_indents();
                            return self.lex();
                        } else {
                            // TODO: I don't know yet how to deal with '...' followed by end-of-file
                            panic!()
                        }
                    }
                }
                let newline_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                if let Match(_) = newline_match_res {
                    // Newline WITHOUT line continuation.
                    // This is a new line, so there may be indents.
                    self.buffer
                        .push(Tokens::EndStatement(EndStatementToken::new_end_line()));
                    self.lex_indents();
                    return self.lex();
                }
                let end_statement_match_res = self.reader.borrow_mut().matches(";");
                if let Match(_) = end_statement_match_res {
                    // Semicolon, which ends a statement.
                    // Need to do some extra work with buffer, because there may be a newline followed by indentation, which ; should precede.
                    self.buffer
                        .push(Tokens::EndStatement(EndStatementToken::new_semicolon()));
                    let end_line_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                    if let Match(_) = end_line_match_res {
                        // If semicolon is followed by a newline (redundant), then we need to deal with indents (but ignore the newline itself).
                        // This will return the queue of tokens, including the semicolon.
                        return self.lex_indents();
                    }
                    // No newline, can just return the semicolon (which is certainly on the queue, and should be the only thing, but it is fine here if not).
                    return Token(self.buffer.pop().unwrap());
                }
                //
                // Indentation done; do the rest of lexing.
                //
                // Parse identifiers and keywords. This assumes that keywords are a subset of identifiers.
                if let Match(word) = self
                    .reader
                    .borrow_mut()
                    .matches(IdentifierToken::subpattern())
                {
                    // later: maybe turn identifier into keyword to avoid a string copy? kind of elaborate...
                    if let Ok(keyword) = KeywordToken::from_str(word.clone()) {
                        return Token(Tokens::Keyword(keyword));
                    }
                    return Token(Tokens::Identifier(IdentifierToken::from_str(word).unwrap()));
                }
                // Literal
                let string_match_res = self.reader.borrow_mut().matches("[a-z]?\"");
                if let Match(_) = string_match_res {
                    let sublexer: Box<Lexer> =
                        Box::new(StringLexer::new_double_quoted(self.reader.clone()));
                    self.reader_or_delegate = ReaderOrDelegate::Delegate(sublexer);
                    return self.lex();
                }
                // Association (before operator)
                let association_match_res = self
                    .reader
                    .borrow_mut()
                    .matches(&AssociationToken::subpattern());
                if let Match(token) = association_match_res {
                    if token.chars().last().unwrap() == '=' {
                        //                        return Token(Tokens::Association(AssociationToken::from_str(token[..1]).unwrap()));
                        return Token(Tokens::Association(AssociationToken::from_unprefixed())); // TODO
                    } else {
                        return Token(Tokens::Association(AssociationToken::from_unprefixed()));
                    }
                }
                // Operator
                let operator_match_res = self
                    .reader
                    .borrow_mut()
                    .matches(OperatorToken::subpattern());
                if let Match(token) = operator_match_res {
                    return Token(Tokens::Operator(OperatorToken::from_str(&token).unwrap()));
                }
                // Grouping symbols
                if let Match(_) = self.reader.borrow_mut().matches(r"\(") {
                    return Token(Tokens::ParenthesisOpen(ParenthesisOpenToken::new()));
                }
                if let Match(_) = self.reader.borrow_mut().matches(r"\)") {
                    return Token(Tokens::ParenthesisClose(ParenthesisCloseToken::new()));
                }

                let unknown_word = self.reader.borrow_mut().matches("[^\\s]+");
                match unknown_word {
                    Match(word) => return Token(Tokens::Unlexable(UnlexableToken::new(word))),
                    NoMatch() => {
                        println!("END {:?}", self.reader.borrow()); // TODO
                        panic!("Do not know how to proceed with parsing")
                    }
                    EOF() => {
                        // TODO: also dedent and end statement here
                        End
                    }
                }
            }
        }
    }

    fn get_reader(&self) -> Rc<RefCell<Reader>> {
        match self.reader_or_delegate {
            ReaderOrDelegate::Reader() => self.reader.clone(),
            ReaderOrDelegate::Delegate(ref delegate) => delegate.get_reader(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CodeLexer;
    use mango::io::fortest::StringReader;
    use mango::lexing::util::lex_all::{lex_all, LexList};
    use mango::token::tokens::AssociationToken;
    use mango::token::tokens::EndBlockToken;
    use mango::token::tokens::EndStatementToken;
    use mango::token::tokens::IdentifierToken;
    use mango::token::tokens::KeywordToken;
    use mango::token::tokens::LiteralToken;
    use mango::token::tokens::OperatorToken;
    use mango::token::tokens::ParenthesisCloseToken;
    use mango::token::tokens::ParenthesisOpenToken;
    use mango::token::tokens::StartBlockToken;
    use mango::token::Tokens;
    use std::cell::RefCell;
    use std::ops::Generator;
    use std::rc::Rc;

    fn assert_text_to_tokens(text: &str, tokens: Vec<Tokens>) {
        assert_eq!(
            LexList::from_tokens(tokens),
            lex_all(&mut CodeLexer::new(Rc::new(RefCell::new(
                StringReader::new(text.to_owned())
            ))))
        )
    }

    #[test]
    fn test_lexing_individual() {
        assert_text_to_tokens(
            "if",
            vec![Tokens::Keyword(
                KeywordToken::from_str("if".to_owned()).unwrap(),
            )],
        );
        // todo: more
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
                Tokens::Operator(OperatorToken::from_str("<").unwrap()),
                Tokens::Literal(LiteralToken::Int(128)),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
                Tokens::StartBlock(StartBlockToken::new()),
                Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap()),
                Tokens::Association(AssociationToken::from_str("+".to_owned()).unwrap()),
                Tokens::Literal(LiteralToken::Int(1)),
                Tokens::EndBlock(EndBlockToken::new(true, false)),
            ],
        );
    }

    #[test]
    fn test_lexing_delegation() {}

    #[test]
    fn generators() {
        let mut gen = || {
            yield Tokens::Keyword(KeywordToken::from_str("let".to_owned()).unwrap());
            yield Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap());
            yield Tokens::Association(AssociationToken::from_unprefixed());
            return;
        };
        let first = unsafe { gen.resume() };
    }
}
