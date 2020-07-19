//use crate::parsing::arithmetic::parse_addition;
use crate::parsing::util::cursor::ParseCursor;
use crate::token::collect::FileTokens;
use crate::token::Tokens;

pub fn parse(file_lex: FileTokens) {
    let mut cursor = ParseCursor::new(file_lex);
    unimplemented!()
    //parse_addition(lex);
}
