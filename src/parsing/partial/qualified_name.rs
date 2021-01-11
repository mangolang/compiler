use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::lexeme::IdentifierLexeme;

/// Parse a qualified name, which is (identifier + period)* + identifier
///
/// Note that field access for a record has the same structure, so context is important:
///
/// * Declarations of modules, records or functions should be simple (without periods)
/// * Names of imports are fully qualified (usually, but not always, containing periods)
/// * Uses of records or functions can either use fully qualified name or simple name.
pub fn parse_qualified_name(mut cursor: ParseCursor) -> ParseRes<IdentifierLexeme> {
    unimplemented!();  //TODO @mark:

    // while let Ok((expr_cursor, expr)) = parse_expression(cursor) {
    //     expressions.push(expr);
    //     let mut separator_cursor = expr_cursor; // copy
    //     match separator_cursor.take() {
    //         Ok(token) => match token {
    //             // There is a separator, continue for another expression.
    //             Lexeme::Comma(_) | Lexeme::Newline(_) => {
    //                 separator_cursor.skip_while(|lexeme| lexeme.is_newline());
    //             }
    //             // No separator, so this is the end of the multi-expression - or a syntax
    //             // error, but that's for the next parser to find out. Revert eating separator.
    //             _not_a_separator => return Ok((expr_cursor, expressions)),
    //         },
    //         Err(_) => {
    //             // Reached the end of input. There should probably be a closing symbol,
    //             // but that is up to the outer parser (which knows what the opening is).
    //             return Ok((expr_cursor, expressions));
    //         }
    //     }
    //     cursor = separator_cursor
    // }
    // // Did not find another expression; apparently the last expression had a
    // // comma/newline, and we are done.
    // Ok((cursor, expressions))
}

//TODO @mark: tests
