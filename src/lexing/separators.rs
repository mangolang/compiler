use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexeme::collect::short::colon;
use crate::lexeme::collect::short::comma;
use crate::lexeme::collect::short::ellipsis;
use crate::lexeme::collect::short::newline;
use crate::lexeme::collect::short::period;
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};

lazy_static! {
    static ref SEPARATOR_RE: Regex = Regex::new("^(\\.\\.\\.|…|\\.|,|:|\r\n|\n|\r)").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the lexemes to the Lexer.
pub fn lex_separators(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    let mut found_newline = false;
    while let ReaderResult::Match(source) = reader.strip_match(&*SEPARATOR_RE) {
        let lexeme = match source.as_str() {
            r"..." | r"…" => ellipsis(source),
            r"." => period(source),
            r"," => comma(source),
            r":" => colon(source),
            "\r\n" | "\n" | "\r" => {
                // Indentation should be parsed after a newline, so stop.
                found_newline = true;
                lexer.set_at_indentable(true);
                newline(source)
            }
            _ => unreachable!(),
        };
        lexer.add(lexeme);
        if found_newline {
            break;
        }
    }
}

#[cfg(test)]
mod grouping {
    use crate::lexeme::collect::for_test::builder;
    use crate::lexeme::Lexeme;
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    use super::lex_separators;

    fn check(input: &str, expected: &[Lexeme]) {
        let expected: LexemeCollector = expected.into();
        let (_source, mut reader, mut lexer) = create_lexer(input);
        lex_separators(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
    }

    #[test]
    fn empty() {
        check("", &[]);
        check("    \t", &[]);
    }

    #[test]
    fn mismatch() {
        check("(", &[]);
        check("0", &[]);
        check("a", &[]);
    }

    #[test]
    fn single_ellipsis() {
        check(r"...", &builder().ellipsis().build());
        check(r"…", &builder().ellipsis().build());
    }

    #[test]
    fn multiple_ellipsis() {
        check(r"...…", &builder().ellipsis().ellipsis().build());
        check(r"…...", &builder().ellipsis().ellipsis().build());
        check(r"……", &builder().ellipsis().ellipsis().build());
        check(r"......", &builder().ellipsis().ellipsis().build());
    }

    #[test]
    fn single_period() {
        check(r".", &builder().period().build());
    }

    #[test]
    fn multiple_period() {
        check(r"..", &builder().period().period().build());
        check(r".. .", &builder().period().period().period().build());
    }

    #[test]
    fn single_comma() {
        check(r",", &builder().comma().build());
    }

    #[test]
    fn multiple_comma() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r",,", &builder().comma().comma().build());
    }

    #[test]
    fn single_colon() {
        check(r":", &builder().colon().build());
    }

    #[test]
    fn multiple_colon() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r"::", &builder().colon().colon().build());
    }

    #[test]
    fn single_newline() {
        check("\r\n", &builder().newline().build());
        check("\n", &builder().newline().build());
        check("\r", &builder().newline().build());
    }

    #[test]
    fn stop_after_newline() {
        check("\r\n:", &builder().newline().build());
        check("\n...", &builder().newline().build());
        check("\r,", &builder().newline().build());
    }

    #[test]
    fn multiple_newline() {
        // Only one newline should be matched
        check("\n\n", &builder().newline().build());
        check("\r\r", &builder().newline().build());
        check("\r\n \r\n", &builder().newline().build());
    }

    #[test]
    fn after_mismatch() {
        check(r"1 …", &[]);
        check(r"1 .", &[]);
        check(r"1 ,", &[]);
        check("1 \r", &[]);
    }

    #[test]
    fn combined_1() {
        check(",....\r\n", &builder().comma().ellipsis().period().newline().build());
    }

    #[test]
    fn combined_2() {
        check("...….,\n,", &builder().ellipsis().ellipsis().period().comma().newline().build());
    }

    #[test]
    fn combined_3() {
        check(
            "...:,\n:",
            &builder().ellipsis().colon().comma().newline().build(),
            // stop after newline
        );
    }

    #[test]
    fn and_words() {
        check(r"...abc", &builder().ellipsis().build());
        check(r".abc", &builder().period().build());
        check(r",abc", &builder().comma().build());
        check(":abc", &builder().colon().build());
        check("\nabc", &builder().newline().build());
    }
}
