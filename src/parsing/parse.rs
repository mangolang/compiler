use crate::lexeme::collect::FileLexemes;
use crate::lexeme::Lexemes;
use crate::parsing::expression::parse_expression;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse(file_lex: FileLexemes) {
    let cursor = ParseCursor::new(&file_lex);
    let parselet = parse_expression(cursor);
    // let parse_parenthese(cursor);
    //parse_addition(lex);
    unimplemented!()
}

