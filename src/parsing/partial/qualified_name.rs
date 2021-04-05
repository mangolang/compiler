use crate::parsing::util::cursor::{ParseCursor, End};
use crate::parsing::util::{ParseRes, NoMatch};
use crate::lexeme::{IdentifierLexeme, Lexeme};

//TODO @mark: tests for FQNs
/// Parse a qualified name, which is (identifier + period)* + identifier
///
/// Note that field access for a record has the same structure, so context is important:
///
/// * Declarations of modules, records or functions should be simple (without periods)
/// * Names of imports are fully qualified (usually, but not always, containing periods)
/// * Uses of records or functions can either use fully qualified name or simple name.
pub fn parse_qualified_name(mut cursor: ParseCursor) -> ParseRes<IdentifierLexeme> {

    if let Lexeme::Identifier(root_iden) = cursor.take()? {
        //TODO @mark: is this clone needed?
        let mut full_name = root_iden.clone();
        let mut tail_cursor = cursor;

        loop {
            match cursor.take() {
                Ok(Lexeme::Period(separator)) => {
                    let separator_copy = separator.clone();
                    if let Lexeme::Identifier(sub_iden) = cursor.take()? {
                        debug_assert!(sub_iden.is_simple());
                        full_name = full_name.join(&separator_copy, sub_iden);
                        tail_cursor = cursor;
                        continue;
                    }
                },
                Ok(_) | Err(End) => {},
            }

            return Ok((tail_cursor, full_name))
        }
    }

    Err(NoMatch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexeme::collect::for_test::builder;
    use crate::common::codeparts::fqn::FQN;

    #[test]
    fn wrong_lexeme() {
        let lexemes = builder()
            .literal_text("hello")
            .build();
        let result = parse_qualified_name(lexemes.cursor());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn leading_separator() {
        let lexemes = builder()
            .period()
            .identifier("hello")
            .build();
        let result = parse_qualified_name(lexemes.cursor());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn trailing_separator() {
        let lexemes = builder()
            .identifier("hello")
            .period()
            .build();
        let result = parse_qualified_name(lexemes.cursor());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn eof_after_name() {
        let lexemes = builder()
            .identifier("hello")
            .build();
        let (cursor, parselets) = parse_qualified_name(lexemes.cursor()).unwrap();
        assert_eq!(FQN::new("hello").unwrap(), parselets.name);
        assert!(cursor.peek().is_err());
    }

    #[test]
    fn single() {
        let lexemes = builder()
            .identifier("hello")
            .literal_int(7)
            .build();
        let (cursor, parselets) = parse_qualified_name(lexemes.cursor()).unwrap();
        assert_eq!(FQN::new("hello").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        assert_eq!(&lexemes[1], next);
    }

    #[test]
    fn two() {
        let lexemes = builder()
            .identifier("my_lib")
            .period()
            .identifier("MyClass")
            .build();
        let (cursor, parselets) = parse_qualified_name(lexemes.cursor()).unwrap();
        assert_eq!(FQN::new("my_lib.MyClass").unwrap(), parselets.name);
        assert!(cursor.peek().is_err());
    }

    #[test]
    fn three() {
        let lexemes = builder()
            .identifier("std")
            .period()
            .identifier("text")
            .period()
            .identifier("regex")
            .literal_int(7)
            .build();
        let (cursor, parselets) = parse_qualified_name(lexemes.cursor()).unwrap();
        assert_eq!(FQN::new("std.text.regex").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        assert_eq!(&lexemes[5], next);
    }
}