use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::io::source::SourceFile;
use crate::lexing::grouping::lex_grouping;
use crate::lexing::indent::lex_indents;
use crate::lexing::operator::lex_operator;
use crate::lexing::lexer::{CodeLexer, Lexer};
use crate::lexing::special::{lex_unlexable, lex_eof};
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::lexing::reader::source_reader::SourceReader;
use crate::token::{Tokens, UnlexableToken};

//TODO performance: one day maybe use arena allocation

macro_rules! try_lex (
    // Invoke a lexing function, and 'continue' if successful.
    // f: The lexing function
    // r: The Reader implementation
    // l: The Lexer implementation
    ( $f: ident, $r: ident, $l: ident ) => {
        {
            let initial_progress = $l.progress();
            $f(&mut $r, &mut $l);
            if ($l.progress() != initial_progress) {
                continue;
            }
        }
    };
);

/// Lexes a whole source file and returns the tokens.
pub fn lex(source: &SourceFile) -> Vec<Tokens> {
    //TODO performance: does this initial capacity make sense?
    let mut reader = SourceReader::new(source);
    let mut lexer = CodeLexer::new(source.len());
    loop {
        try_lex!(lex_indents, reader, lexer);
        try_lex!(lex_grouping, reader, lexer);
        try_lex!(lex_operator, reader, lexer);
        //try_lex!(lex_identifier, reader, lexer);
        if lex_eof(&mut reader) { break }
        try_lex!(lex_unlexable, reader, lexer);
    }
    lexer.into_tokens()
}

#[cfg(test)]
mod try_lex {
    use crate::io::source::SourceFile;
    use crate::lexing::reader::reader::Reader;
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::token::UnlexableToken;

    use super::*;
    use crate::lexing::tests::create_lexer;

    fn lex_fn_match(reader: &mut impl Reader, lexer: &mut impl Lexer) {
        lexer.add(Tokens::Unlexable(UnlexableToken::new("hi".to_owned())))
    }

    fn lex_fn_no_match(reader: &mut impl Reader, lexer: &mut impl Lexer) {}

    #[test]
    fn test_match() {
        let (source, mut reader, mut lexer) = create_lexer("");
        for i in 0..3 {
            try_lex!(lex_fn_match, reader, lexer);
            panic!("Execution should not have reached here, because every iteration matches and should do 'continue'");
        }
    }

    #[test]
    fn test_no_match() {
        let (source, mut reader, mut lexer) = create_lexer("");
        let mut end_of_loop_count = 0;
        for i in 0..3 {
            try_lex!(lex_fn_no_match, reader, lexer);
            end_of_loop_count += 1;
        }
        assert_eq!(end_of_loop_count, 3, "Execution should have reached end of loop, because there is not 'continue' on mismatch");
    }
}
