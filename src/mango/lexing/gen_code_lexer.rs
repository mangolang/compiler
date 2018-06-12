use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::string_lexer::StringLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::special::UnlexableToken;
use mango::token::Tokens;
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
use mango::util::collection::Queue;
use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
use std::rc::Rc;

/// This generator does the real lexing work, but is wrapped in a normal
/// class to satisfy an interface that doesn't expose nightly or unsafe features.
//struct GenCodeLexer<G: Generator<Yield = Tokens, Return = ()>> {
//    generator: G
//}
//
//impl<G: Generator<Yield = Tokens, Return = ()>> GenCodeLexer<G> {
//    pub fn new() -> Self {
//        let mut reader: Rc<RefCell<Reader>>;
//        GenCodeLexer{ generator: 0 }
//    }
//}

struct Container<G: Generator<Yield = Tokens, Return = ()>> {
    generator: G,
}

impl Container<Box<Generator<Yield = Tokens, Return = ()>>> {
    pub fn new(reader: Box<Reader>) -> Box<Self> {
        let q = 42;
        Box::new(Container {
            generator: Box::new(move || {

                // If there is a buffer due to indentation or continuations, return from that.
                if let Some(token) = self.buffer.pop() {
                    yield token;
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
                            yield self.lex();
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
                    yield self.lex();
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
                        yield self.lex_indents();
                    }
                    // No newline, can just return the semicolon (which is certainly on the queue, and should be the only thing, but it is fine here if not).
                    yield self.buffer.pop().unwrap();
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
                        yield Tokens::Keyword(keyword);
                    }
                    yield Tokens::Identifier(IdentifierToken::from_str(word).unwrap());
                }
                // Literal
                let string_match_res = self.reader.borrow_mut().matches("[a-z]?\"");
                if let Match(_) = string_match_res {
                    let sublexer: Box<Lexer> =
                        Box::new(StringLexer::new_double_quoted(self.reader.clone()));
                    self.reader_or_delegate = ReaderOrDelegate::Delegate(sublexer);
                    yield self.lex();
                }
                // Association (before operator)
                let association_match_res = self
                    .reader
                    .borrow_mut()
                    .matches(&AssociationToken::subpattern());
                if let Match(token) = association_match_res {
                    if token.chars().last().unwrap() == '=' {
                        //                        return Token(Tokens::Association(AssociationToken::from_str(token[..1]).unwrap()));
                        yield Tokens::Association(AssociationToken::from_unprefixed()); // TODO
                    } else {
                        yield Tokens::Association(AssociationToken::from_unprefixed());
                    }
                }
                // Operator
                let operator_match_res = self
                    .reader
                    .borrow_mut()
                    .matches(OperatorToken::subpattern());
                if let Match(token) = operator_match_res {
                    yield Tokens::Operator(OperatorToken::from_str(&token).unwrap());
                }
                // Grouping symbols
                if let Match(_) = self.reader.borrow_mut().matches(r"\(") {
                    yield Tokens::ParenthesisOpen(ParenthesisOpenToken::new());
                }
                if let Match(_) = self.reader.borrow_mut().matches(r"\)") {
                    yield Tokens::ParenthesisClose(ParenthesisCloseToken::new());
                }

                let unknown_word = self.reader.borrow_mut().matches("[^\\s]+");
                match unknown_word {
                    Match(word) => yield Tokens::Unlexable(UnlexableToken::new(word)),
                    NoMatch() => {
                        panic!("Do not know how to proceed with parsing")
                    }
                    EOF() => {
                        // TODO: also dedent and end statement here
                        return
                    }
                }

            }),
        })
    }

//    pub fn next(&mut self) -> Option<i32> {
//        // Hide the unsafe part.
//        match unsafe { self.generator.resume() } {
//            GeneratorState::Yielded(nr) => Option::Some(nr),
//            GeneratorState::Complete(_) => Option::None,
//        }
//    }
}
