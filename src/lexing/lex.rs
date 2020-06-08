use crate::token::Tokens;
use crate::io::source::SourceFile;
use regex::internal::Input;

//TODO @mark: check regexes in a unit test (make sure they compile and start with ^)
//TODO @mark: note that regexes should strip whitespace themselves if needed

//TODO performance: one day maybe use arena allocation

/// Lexes a whole source file and returns the tokens.
pub fn lex(source: &SourceFile) -> Vec<Tokens> {
    //TODO performance: does this initial capacity make sense?
    let mut tokens = Vec::with_capacity(source.len() / 3);

}
