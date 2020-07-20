// use crate::lexing::util::lex_list::LexList;
//
// pub fn parse_literal(lex: LexList) {}

use crate::parsing::util::cursor::ParseCursor;
use crate::lexeme::Tokens;

pub fn parse_literal(mut cursor: ParseCursor) -> Option<Tokens> {
    match cursor.take() {
        Some(token) => {
            if let Tokens::Literal(literal) = token {
                Some(literal)
            } else {
                None
            }
        },
        None => None,
    }
}
