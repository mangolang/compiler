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
use std::borrow::BorrowMut;
use mango::util::strslice::charsliceto;
use mango::util::strslice::slice::glyphat;

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
    delegate: Option<Box<Lexer>>,
    reader: Rc<RefCell<Reader>>,
    generator: G,
}

impl Container<Box<Generator<Yield = Tokens, Return = ()>>> {

    fn lex_indents(&mut self) -> Vec<Tokens> {
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

    pub fn new(&mut self, reader: Rc<RefCell<Reader>>) -> Box<Self> {
        let q = 42;
        Box::new(Container {
            reader: reader,
            delegate: Option::None,
            generator: Box::new(move || {

                loop {

                    // Delegate to another lexer if one is set.
                    if let Option::Some(delegate) = self.delegate {
                        match delegate.lex() {
                            MaybeToken::Token(token) => {
                                yield token;
                                continue;
                            }
                            MaybeToken::End => {
                                self.delegate = Option::None;
                            }
                        }
                    }

                    // TODO: see if all these match_res can be removed (they couldn't before due to borrowchecker, even with non-lexical lifetimes)
                    let continue_match_res = self.reader.borrow_mut().matches("\\.\\.\\.");
                    if let Match(_) = continue_match_res {
                        // Line continuation has no token, it just continues on the next line.
                        let newline_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                        if let Match(_) = newline_match_res {
                            // There should always be a newline after continuations, so that they can be ignored together.
                        } else {
                            // All the text between ... and the end of the line is unlexable.
                            let newline_match_res = self.reader.borrow_mut().matches("[^\\n]*\\n\\r?");
                            if let Match(word) = newline_match_res {
                                yield Tokens::Unlexable(UnlexableToken::new(word));
                                // This is a new line, so there may be indents.
                                // TODO: is there any yield-from like Python?
                                for res in self.lex_indents() {
                                    yield res;
                                }
                            } else {
                                // TODO: I don't know yet how to deal with '...' followed by end-of-file
                                panic!()
                            }
                        }
                        // TODO: are continues necessary? it seems more state-independent to restart for each token
                        continue;
                    }
                    let newline_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                    if let Match(_) = newline_match_res {
                        // Newline WITHOUT line continuation.
                        // This is a new line, so there may be indents.
                        yield Tokens::EndStatement(EndStatementToken::new_end_line());
                        for res in self.lex_indents() {
                            yield res;
                        }
                        continue;
                    }
                    let end_statement_match_res = self.reader.borrow_mut().matches(";");
                    if let Match(_) = end_statement_match_res {
                        // Semicolon, which ends a statement.
                        // Need to do some extra work with buffer, because there may be a newline followed by indentation, which ; should precede.
                        yield Tokens::EndStatement(EndStatementToken::new_semicolon());
                        let end_line_match_res = self.reader.borrow_mut().matches("\\n\\r?");
                        if let Match(_) = end_line_match_res {
                            // If semicolon is followed by a newline (redundant), then we need to deal with indents (but ignore the newline itself).
                            // This will return the queue of tokens, including the semicolon.
                            for res in self.lex_indents() {
                                yield res;
                            }
                        }
                        // No newline, can just return the semicolon (which is certainly on the queue, and should be the only thing, but it is fine here if not).
                        continue;
                    }

                    //
                    // Indentation done; do the rest of lexing.
                    //
                    // Parse identifiers and keywords. This assumes that keywords are a subset of identifiers.
                    let word_match_res = self.reader.borrow_mut().matches(IdentifierToken::subpattern());
                    if let Match(word) = word_match_res {
                        // Check if it is a keyword.
                        // TODO: maybe turn identifier into keyword to avoid a string copy? kind of elaborate...
                        if let Ok(keyword) = KeywordToken::from_str(word.clone()) {
                            yield Tokens::Keyword(keyword);
                        }
                        yield Tokens::Identifier(IdentifierToken::from_str(word).unwrap());
                        continue;
                    }
                    // String literal (delegated).
                    let string_match_res = self.reader.borrow_mut().matches("[a-z]?\"");
                    if let Match(_) = string_match_res {
                        let sublexer: Box<Lexer> = Box::new(StringLexer::new_double_quoted(self.reader.clone()));
                        self.delegate = Option::Some(sublexer);
                        continue;
                    }
                    // Association (before operator).
                    let association_match_res = self.reader.borrow_mut().matches(&AssociationToken::subpattern());
                    if let Match(token) = association_match_res {
                        if glyphat(token, -1) == "=" {
                            yield Tokens::Association(AssociationToken::from_unprefixed()); // TODO
                        } else {
                            yield Tokens::Association(AssociationToken::from_str(charsliceto(token, -1)).unwrap());
                        }
                        continue;
                    }
                    // Operator.
                    let operator_match_res = self.reader.borrow_mut().matches(OperatorToken::subpattern());
                    if let Match(token) = operator_match_res {
                        yield Tokens::Operator(OperatorToken::from_str(&token).unwrap());
                        continue;
                    }
                    // Grouping symbols
                    if let Match(_) = self.reader.borrow_mut().matches(r"\(") {
                        yield Tokens::ParenthesisOpen(ParenthesisOpenToken::new());
                        continue;
                    }
                    if let Match(_) = self.reader.borrow_mut().matches(r"\)") {
                        yield Tokens::ParenthesisClose(ParenthesisCloseToken::new());
                        continue;
                    }


                    let unknown_word = self.reader.borrow_mut().matches("[^\\s]+");
                    match unknown_word {
                        Match(word) => yield Tokens::Unlexable(UnlexableToken::new(word)),
                        NoMatch() => panic!("Do not know how to proceed with parsing"),
                        EOF() => {
                            // TODO: also dedent and end statement here
                            return
                        }
                    }
                    continue;
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
