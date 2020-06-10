use crate::io::source::SourceFile;
use crate::lexing::indent::lex_indents;
use crate::lexing::lexer::{CodeLexer, Lexer};
use crate::lexing::reader::source_reader::SourceReader;
use crate::token::Tokens;

//TODO @mark: check regexes in a unit test (make sure they compile and start with ^)
//TODO @mark: note that regexes should strip whitespace themselves if needed

//TODO performance: one day maybe use arena allocation

/// Lexes a whole source file and returns the tokens.
pub fn lex(source: &SourceFile) -> Vec<Tokens> {
    //TODO performance: does this initial capacity make sense?
    let mut reader = SourceReader::new(source);
    let mut lexer = CodeLexer::new(source.len());
    lex_indents(&mut reader, &mut lexer);
    lex_grouping(&mut reader, &mut lexer);

    lexer.into_tokens()
}
