use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::util::strslice::char_ops::CharOps;

lazy_static! {
    // From a single quote until the first non-escaped single-quote on the same line.
    // TODO: Only single-quoted, single-line strings for now; double-quoted strings may become templates?
    //TODO @mark: doesn't work if the escape is escaped, i.e. \\' (the \ escapes the other \, while ' is normal)
    pub static ref SINGLE_QUOTE_RE: Regex = Regex::new(r"^(?:''|'[^\n\r]*?[^\\]')").unwrap();
}

/// Convert a single-quoted string to the text it contains.
pub fn parse_single_quote(text: &str) -> String {
    debug_assert!(text.starts_with('\''));
    debug_assert!(text.ends_with('\''));
    let unquoted = &text[1..text.len() - 1];
    unquoted.to_owned()
}

#[cfg(test)]
mod single_quoted {
    use super::parse_single_quote;

    //TODO @mark: test coverage could probably be better, i.e.
    //TODO @mark: - escaping of quotes
    //TODO @mark: - escaping of escapes (\\)
    //TODO @mark: - specials like \t \n
    //TODO @mark: - cover more invalid cases? or only those that I think could match the regex?

    #[test]
    fn no_content() {
        assert_eq!(&parse_single_quote("''"), "");
    }

    #[test]
    fn simple() {
        assert_eq!(&parse_single_quote("'x'"), "x");
        assert_eq!(&parse_single_quote("'hello world!'"), "hello world!");
    }

    #[test]
    fn double_quotes() {
        assert_eq!(&parse_single_quote("'\"\"'"), "\"\"");
    }

    //#[test]  //TODO @mark: support this
    fn escaped() {
        assert_eq!(&parse_single_quote("'\\''"), "'");
    }

    #[test]
    fn escape_escaped() {
        assert_eq!(&parse_single_quote("'\\\\'"), "\\\\");
    }
}
