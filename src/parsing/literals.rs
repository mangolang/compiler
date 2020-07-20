// use crate::lexing::util::lex_list::LexList;
//
// pub fn parse_literal(lex: LexList) {}

use crate::parsing::util::cursor::ParseCursor;
use crate::lexeme::Lexemes;

pub fn parse_literal(mut cursor: ParseCursor) -> Option<Lexemes> {
    match cursor.take() {
        Some(lexeme) => {
            if let Lexemes::Literal(literal) = lexeme {
                Some(literal)
            } else {
                None
            }
        },
        None => None,
    }
}
