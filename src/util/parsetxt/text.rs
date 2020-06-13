use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::util::strslice::char_ops::CharOps;

lazy_static! {
    // From a single quote until the first non-escaped single-quote on the same line.
    // TODO: Only single-quoted, single-line strings for now; double-quoted strings may become templates?
    //TODO @mark: doesn't work if the escape is escaped, i.e. \\' (the \ escapes the other \, while ' is normal)
    pub static ref SINGLE_QUOTE_RE: Regex = Regex::new(r"^(?:'[^\n\r]*?[^\\]'|'')").unwrap();
}

/// Convert a single-quoted string to the text it contains.
pub fn parse_single_quote(text: &str) -> String {
    let unquoted = &text[1 .. text.len() - 1];
    unquoted.to_owned()
}

#[cfg(test)]
mod single_quoted {
    use super::parse_single_quote;

    #[test]
    fn simple() {
        todo!();  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}
