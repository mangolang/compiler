use crate::parselet::ExpressionParselets;
use crate::parsing::expression::map_literal::parse_map_literal;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

//TODO @mark: perhaps merge with map_literal
pub fn parse_object_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    // let mut literal_cursor = cursor.fork();
    // if let Lexeme::Literal(literal_lexeme) = literal_cursor.take()? {
    //     let literal = literal_lexeme.clone();
    //     return Ok((literal_cursor, ExpressionParselets::Literal(LiteralParselet::new(literal))));
    // }
    // parse_parenthesised_group(cursor)
    dbg!("TO DO: parse_object_literal");
    parse_map_literal(cursor)
}

#[cfg(test)]
mod maps {
    use ::ustr::ustr;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::FileLexemes;
    use crate::lexeme::collect::for_test::builder;
    use crate::lexeme::LiteralLexeme;
    use crate::parselet::short::literal;

    use super::*;

    fn check(_lexemes: FileLexemes, _expected: ExpressionParselets) {
        // let cursor = lexemes.cursor();
        // let (cursor, parselet) = parse_literal(cursor).unwrap();
        // assert_eq!(expected, parselet);
        // assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            builder().literal_text("hello42").file(),
            literal(LiteralLexeme::new_text(ustr("hello42"), SourceSlice::mock())),
        );
        todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}
