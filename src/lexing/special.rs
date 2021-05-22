use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexeme::{Lexeme, UnlexableLexeme};
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};

lazy_static! {
    static ref SINGLE_RE: Regex = Regex::new(r"(?s)^.").unwrap();
    static ref EMPTY_RE: Regex = Regex::new(r"^[ \t\n\r]*").unwrap();
}

/// Lex a single symbol as unlexable. Should only be used if the lexer is stuck, to unstuck it.
pub fn lex_unlexable(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    let source = reader.strip_match(&*SINGLE_RE).unwrap();
    lexer.add(Lexeme::Unlexable(UnlexableLexeme::from_source(source)));
}

/// Check whether the end of files has been reached (true if EOF).
pub fn lex_eof(reader: &mut impl Reader) -> bool {
    if let ReaderResult::Match(whitespace) = reader.direct_peek(&*EMPTY_RE) {
        debug_assert!(whitespace.len() <= reader.remaining_len());
        if whitespace.len() == reader.remaining_len() {
            // End of files has been reached (the rest is all whitespace)
            return true;
        }
    }
    false
}

#[cfg(test)]
mod unlexable {
    use crate::io::source::SourceFile;
    use crate::lexeme::collect::short::unlexable;
    use crate::lexeme::collect::FileLexemes;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    use super::lex_unlexable;

    #[test]
    fn letter() {
        let (_source, mut reader, mut lexer) = create_lexer("abc");
        lex_unlexable(&mut reader, &mut lexer);
        let expected = FileLexemes::new(vec![unlexable(SourceFile::mock("a").slice(0, 1))]);
        assert_eq!(lexer.into_lexemes(), expected);
    }

    #[test]
    fn newline() {
        // Newline is a partial case, because normally regex's '.' does not match it.
        let (_source, mut reader, mut lexer) = create_lexer("\nabc");
        lex_unlexable(&mut reader, &mut lexer);
        let expected = FileLexemes::new(vec![unlexable(SourceFile::mock("\n").slice(0, 1))]);
        assert_eq!(lexer.into_lexemes(), expected);
    }
}

#[cfg(test)]
mod eof {
    use crate::lexing::special::lex_eof;
    use crate::lexing::tests::create_lexer;

    #[test]
    fn empty() {
        let (_, mut reader, _) = create_lexer("");
        assert!(lex_eof(&mut reader));
    }

    #[test]
    fn whitespace() {
        let (_, mut reader, _) = create_lexer(" \n\t ");
        assert!(lex_eof(&mut reader));
    }

    #[test]
    fn just_letter() {
        let (_, mut reader, _) = create_lexer("*");
        assert!(!lex_eof(&mut reader));
    }

    #[test]
    fn whitespace_text() {
        let (_, mut reader, _) = create_lexer(" \n\t abc");
        assert!(!lex_eof(&mut reader));
    }
}
