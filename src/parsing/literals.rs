use crate::lexeme::Lexemes;
use crate::lexeme::LiteralLexeme;
use crate::parselet::ExpressionParselets;
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_literal(cursor: &mut ParseCursor) -> Option<ExpressionParselets> {
    match cursor.take() {
        Some(lexeme) => {
            if let Lexemes::Literal(literal_lexeme) = lexeme {
                Some(ExpressionParselets::Literal(LiteralParselet::new(literal_lexeme.clone())))
            } else {
                None
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod literal {
    use super::*;
    use crate::lexeme::collect::{FileLexemes, literal_text};
    use crate::parselet::short::literal;

    #[test]
    fn text() {
        let lexemes = vec![literal_text("hello42")].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert_eq!(None, cursor.peek());
        let expected = literal(LiteralLexeme::Text("hello42".to_owned()));
        assert_eq!(expected, parselet.unwrap());
        unimplemented!()
    }
}
