use crate::parsing::grouping::parse_parenthese;
use crate::parsing::literals::parse_literal;
//use crate::parsing::arithmetic::parse_addition;
use crate::parsing::util::cursor::ParseCursor;
use crate::token::collect::FileTokens;
use crate::token::Tokens;
use crate::util::parsetxt::int::parse_int;

pub fn parse(file_lex: FileTokens) {
    let mut cursor = ParseCursor::new(&file_lex);
    let parselet = parse_literal(cursor);
    // let parse_parenthese(cursor);
    //parse_addition(lex);
    unimplemented!()
}
