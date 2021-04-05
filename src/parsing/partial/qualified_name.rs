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
            //TODO @mark: do I want to change identifiers to slashes? nice for imports, but inconsistent with in-code use
            match cursor.take() {
                Ok(Lexeme::Operator(operator)) => {
                    if operator.is_import_separator() {
                        let period = operator.clone();  //TODO @mark: get rid of this clone?
                        if let Lexeme::Identifier(sub_iden) = cursor.take()? {
                            full_name = full_name.join(&period, sub_iden);
                            tail_cursor = cursor;
                            continue;
                        }
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
    use crate::lexeme::collect::for_test::{literal_text, identifier, literal_int, period};
    use crate::common::codeparts::fqn::FQN;

    #[test]
    fn wrong_lexeme() {
        let lexemes = vec![literal_text("hello").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let result = parse_qualified_name(cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn leading_separator() {
        let lexemes = vec![period(), identifier("hello").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let result = parse_qualified_name(cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn trailing_separator() {
        let lexemes = vec![identifier("hello").into(), period()].into();
        let cursor = ParseCursor::new(&lexemes);
        let result = parse_qualified_name(cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoMatch);
    }

    #[test]
    fn eof_after_name() {
        let lexemes = vec![identifier("hello").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselets) = parse_qualified_name(cursor).unwrap();
        assert_eq!(FQN::new("hello").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        let q: Lexeme = period().into();
        assert_eq!(q, *next);
    }

    #[test]
    fn single() {
        let lexemes = vec![identifier("root").into(), literal_int(7).into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselets) = parse_qualified_name(cursor).unwrap();
        assert_eq!(FQN::new("root").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        let q: Lexeme = literal_int(7).into();
        assert_eq!(q, *next);
    }

    #[test]
    fn two() {
        let lexemes = vec![identifier("my_lib").into(), period(), identifier("MyClass").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselets) = parse_qualified_name(cursor).unwrap();
        assert_eq!(FQN::new("std.text.regex").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        let q: Lexeme = literal_int(7).into();
        assert_eq!(q, *next);
    }

    #[test]
    fn three() {
        let lexemes = vec![identifier("std").into(), period(), identifier("text").into(), period(), identifier("regex").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselets) = parse_qualified_name(cursor).unwrap();
        assert_eq!(FQN::new("std.text.regex").unwrap(), parselets.name);
        let next = cursor.peek().unwrap();
        let q: Lexeme = literal_int(7).into();
        assert_eq!(q, *next);
    }
}