use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::string_lexer::StringLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::special::UnlexableToken;
use mango::token::tokens::EndBlockToken;
use mango::token::tokens::EndStatementToken;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::KeywordToken;
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
    Reader(Rc<RefCell<Reader>>),
    Delegate(Box<Lexer>),
}

pub struct CodeLexer {
    //    reader: Rc<RefCell<Reader>>,
    indent: i32,

    // This delegate deals with nested structures, like string literals and comments.
    reader_or_delegate: ReaderOrDelegate,
    // This is unfortunate, would not be needed with 'yield' but is now for indents.
    buffer: Queue<Tokens>,
}

impl CodeLexer {
    fn new(reader: Rc<RefCell<Reader>>) -> Self {
        CodeLexer {
            reader_or_delegate: ReaderOrDelegate::Reader(reader),
            indent: 0,
            buffer: Queue::new(),
        }
    }

    fn lex_indents(&mut self, reader: &mut Reader) -> MaybeToken {
        let mut line_indent = 0;
        while let Match(_) = reader.matches("\\t") {
            line_indent += 1;
        }
        for _ in line_indent..self.indent {
            // This line is dedented, make end tokens.
            // TODO: turn this "new" into a constant
            if let Match(_) = reader.matches("end") {
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
    fn lex(&mut self) -> MaybeToken {
        use self::MaybeToken::*;

        // If currently delegating to a sub-lexer, return from that.
        match self.reader_or_delegate {
            ReaderOrDelegate::Delegate(ref mut delegate) => {
                let delegated_token = delegate.lex();
                match delegated_token {
                    End => {
                        // Swap back from delegation to direct mode.
                        let reader = delegate.get_reader().clone();
                        self.reader_or_delegate = ReaderOrDelegate::Reader(reader);
                        self.lex()
                    }
                    Token(token) => Token(token),
                }
                // Code to stop delegation cannot be here, because `self` is still mutably borrowed through `delegate`
            }
            ReaderOrDelegate::Reader(ref mut reader_refcell) => {
                let mut reader = reader_refcell.borrow_mut();
                // todo: maybe this branch could be a separate function?

                // If there is a buffer due to indentation or continuations, return from that.
                if let Some(token) = self.buffer.pop() {
                    return Token(token);
                }
                // Past this point, we assume that hte buffer is empty. When adding stuff, pop it or re-enter lex() soon.
                if let Match(_) = reader.matches("\\.\\.\\.") {
                    // Line continuation has no token, it just continues on the next line.
                    if let Match(_) = reader.matches("\\n\\r?") {
                        // There should always be a newline after continuations, so that they can be ignored together.
                    } else if let Match(word) = reader.matches("[^\\n]*\\n\\r?") {
                        return Token(Tokens::Unlexable(UnlexableToken::new(word)));
                    } else {
                        // TODO: I don't know yet how to deal with '...' followed by end-of-file
                        panic!()
                    }
                    // This is a new line, so there may be indents.
                    return self.lex_indents(&mut *reader);
                }
                if let Match(_) = reader.matches("\\n\\r?") {
                    // Newline WITHOUT line continuation.
                    return Token(Tokens::EndStatement(EndStatementToken::new_end_line()));
                }
                if let Match(_) = reader.matches(";") {
                    // Semicolon, which ends a statement.
                    // Need to do some extra work with buffer, because there may be a newline followed by indentation, which ; should precede.
                    self.buffer
                        .push(Tokens::EndStatement(EndStatementToken::new_semicolon()));
                    if let Match(_) = reader.matches("\\n\\r?") {
                        // If semicolon is followed by a newline (redundant), then we need to deal with indents (but ignore the newline itself).
                        // This will return the queue of tokens, including the semicolon.
                        return self.lex_indents(&mut *reader);
                    }
                    // No newline, can just return the semicolon (which is certainly on the queue, and should be the only thing, but it is fine here if not).
                    return Token(self.buffer.pop().unwrap());
                }
                //
                // Indentation done; do the rest of lexing.
                //
                // Parse identifers and keywords. This assumes that keywords are a subset of identifiers.
                if let Match(word) = reader.matches(IdentifierToken::subpattern()) {
                    // later: maybe turn identifier into keyword to avoid a string copy? kind of elaborate...
                    if let Ok(keyword) = KeywordToken::from_str(word.clone()) {
                        return Token(Tokens::Keyword(keyword));
                    }
                    return Token(Tokens::Identifier(IdentifierToken::from_str(word).unwrap()));
                }
                // Literal
                if let Match(_) = reader.matches("[a-z]?\"") {
                    let sublexer: Box<Lexer> =
                        Box::new(StringLexer::new_double_quoted(reader_refcell.clone()));
                    self.reader_or_delegate = ReaderOrDelegate::Delegate(sublexer);
                    return self.lex();
                }
                // Operator
                // todo
                // Association
                // todo
                // Grouping symbols
                if let Match(_) = reader.matches("(") {
                    return Token(Tokens::ParenthesisOpen(ParenthesisOpenToken::new()));
                }
                if let Match(_) = reader.matches(")") {
                    return Token(Tokens::ParenthesisClose(ParenthesisCloseToken::new()));
                }

                // TODO: specify the unlexable word
                return Token(Tokens::Unlexable(UnlexableToken::new("TODO".to_owned())));
            }
        }
    }

    fn get_reader(&self) -> Rc<RefCell<Reader>> {
        match self.reader_or_delegate {
            ReaderOrDelegate::Reader(ref reader) => reader.clone(),
            ReaderOrDelegate::Delegate(ref delegate) => delegate.get_reader(),
        }
    }
}
