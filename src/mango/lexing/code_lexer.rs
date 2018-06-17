use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::string_lexer::StringLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::lexing::typ::SubLexer;
use mango::lexing::typ::SubLexerResult;
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

pub struct CodeLexer {
    indent: i32,
    buffer: Queue<Tokens>,
}

// TODO: keep the regexes in thread local global scope storage

impl CodeLexer {
    pub fn new() -> Self {
        CodeLexer {
            indent: 0,
            buffer: Queue::new(),
        }
    }

    fn lex_indents(&mut self, reader: &mut Box<Reader>) -> Vec<Tokens> {
        let mut line_indent = 0;
        while let Match(_) = reader.matches("\\t") {
            line_indent += 1;
        }
        let mut tokens: Vec<Tokens> = Vec::with_capacity(8);
        if line_indent < self.indent {
            if let Match(_) = reader.matches(r"end\s") {
                // If this is followed by an 'end' keyword, then that 'end' is redundant.
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, true)));
            } else {
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
            }
            for _ in line_indent..(self.indent - 1) {
                // This line is dedented, make end tokens.
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
            }
        }
        for _ in self.indent..line_indent {
            // This line is indented, make start tokens.
            // TODO: increasing indent by more than one should be a warning
            self.buffer.push(Tokens::StartBlock(StartBlockToken::new()));
        }
        self.indent = line_indent;
        tokens
    }

    fn token_and_indents(&mut self, reader: &mut Box<Reader>, token: Tokens) -> SubLexerResult {
        let mut tokens: Vec<Tokens> = vec![token];
        // This is a new line, so there may be indents.
        tokens.append(&mut self.lex_indents(reader));
        return SubLexerResult::Result(tokens);
    }
}

impl SubLexer for CodeLexer {
    fn lex_pass(&mut self, reader: &mut Box<Reader>) -> SubLexerResult {
        use self::SubLexerResult::*;

        // TODO: put all these match results inline

        // End of line continuation
        if let Match(_) = reader.matches(r"\.\.\.") {
            // Line continuation has no token, it just continues on the next line, ignoring indents (for now).
            if let Match(_) = reader.matches(r"\n\r?\t*") {
                // There should always be a newline after continuations, so that they can be ignored together.
            } else {
                // The rest of this line is unparsable.
                if let Match(word) = reader.matches("[^\\n]*\\n\\r?") {
                    // This is a new line, so there may be indents.
                    return self
                        .token_and_indents(reader, Tokens::Unlexable(UnlexableToken::new(word)));
                } else {
                    // TODO: I don't know yet how to deal with '...' followed by end-of-file
                    panic!()
                }
            }
        }
        // Newlines
        if let Match(_) = reader.matches("\\n\\r?") {
            // Newline WITHOUT line continuation.
            // This is a new line, so there may be indents.
            return self.token_and_indents(
                reader,
                Tokens::EndStatement(EndStatementToken::new_end_line()),
            );
        }
        // End of statement
        if let Match(_) = reader.matches(";") {
            // Semicolon, which ends a statement.
            if let Match(_) = reader.matches("\\n\\r?") {
                // If semicolon is followed by a newline, it is redundant. Deal with indents (but ignore the newline itself).
                return self.token_and_indents(
                    reader,
                    Tokens::EndStatement(EndStatementToken::new_semicolon()),
                );
            } else {
                return SubLexerResult::single(Tokens::EndStatement(
                    EndStatementToken::new_semicolon(),
                ));
            }
        }
        //
        // Indentation done; do the rest of lexing.
        //
        // Parse identifiers and keywords. This assumes that keywords are a subset of identifiers.
        if let Match(word) = reader.matches(IdentifierToken::subpattern()) {
            // TODO: maybe turn identifier into keyword to avoid a string copy? kind of elaborate...
            if let Ok(keyword) = KeywordToken::from_str(word.clone()) {
                return SubLexerResult::single(Tokens::Keyword(keyword));
            }
            return SubLexerResult::single(Tokens::Identifier(
                IdentifierToken::from_str(word).unwrap(),
            ));
        }
        //        // Literal
        //        let string_match_res = reader.matches("[a-z]?\"");
        //        if let Match(_) = string_match_res {
        //            let sublexer: Box<Lexer> =
        //                Box::new(StringLexer::new_double_quoted(self.reader.clone()));
        //            self.reader_or_delegate = ReaderOrDelegate::Delegate(sublexer);
        //            return self.lex();
        //        }
        //        // Association (before operator)
        //        let association_match_res = self
        //            .reader
        //            .borrow_mut()
        //            .matches(&AssociationToken::subpattern());
        //        if let Match(token) = association_match_res {
        //            if token.chars().last().unwrap() == '=' {
        //                //                        return Token(Tokens::Association(AssociationToken::from_str(token[..1]).unwrap()));
        //                return Token(Tokens::Association(AssociationToken::from_unprefixed())); // TODO
        //            } else {
        //                return Token(Tokens::Association(AssociationToken::from_unprefixed()));
        //            }
        //        }
        //        // Operator
        //        let operator_match_res = self
        //            .reader
        //            .borrow_mut()
        //            .matches(OperatorToken::subpattern());
        //        if let Match(token) = operator_match_res {
        //            return Token(Tokens::Operator(OperatorToken::from_str(&token).unwrap()));
        //        }
        //        // Grouping symbols
        //        if let Match(_) = reader.matches(r"\(") {
        //            return Token(Tokens::ParenthesisOpen(ParenthesisOpenToken::new()));
        //        }
        //        if let Match(_) = reader.matches(r"\)") {
        //            return Token(Tokens::ParenthesisClose(ParenthesisCloseToken::new()));
        //        }
        //
        //        let unknown_word = reader.matches("[^\\s]+");
        //        match unknown_word {
        //            Match(word) => return Token(Tokens::Unlexable(UnlexableToken::new(word))),
        //            NoMatch() => {
        //                println!("END {:?}", self.reader.borrow()); // TODO
        //                panic!("Do not know how to proceed with parsing")
        //            }
        //            EOF() => {
        //                // TODO: also dedent and end statement here
        //                End
        //            }
        //        }

        panic!() // TODO TMP
    }
}
