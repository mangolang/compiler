use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{Tokens, ParenthesisOpenToken, ParenthesisCloseToken, UnlexableToken};

lazy_static! {
    static ref SINGLE_RE: Regex = Regex::new(r"(?s)^.").unwrap();
}

/// Lex a single symbol as unlexable. Should only be used if the lexer is stuck, to unstuck it.
pub fn lex_unlexable(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    let chr = reader.strip_match(&*SINGLE_RE).unwrap().as_str().to_owned();
    lexer.add(Tokens::Unlexable(UnlexableToken::new(chr)));
}

#[cfg(test)]
mod unlexable {
    use super::lex_unlexable;
    use crate::lexing::tests::create_lexer;
    use crate::lexing::lexer::Lexer;
    use crate::token::{UnlexableToken, Tokens};

    #[test]
    fn letter() {
        let (source, mut reader, mut lexer) = create_lexer("abc");
        lex_unlexable(&mut reader, &mut lexer);
        assert_eq!(lexer.into_tokens(), vec![Tokens::Unlexable(UnlexableToken::new("a".to_owned()))]);
    }

    #[test]
    fn newline() {
        // Newline is a special case, because normally regex's '.' does not match it.
        let (source, mut reader, mut lexer) = create_lexer("\nabc");
        lex_unlexable(&mut reader, &mut lexer);
        assert_eq!(lexer.into_tokens(), vec![Tokens::Unlexable(UnlexableToken::new("\n".to_owned()))]);
    }
}