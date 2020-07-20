use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexemes};
use crate::lexeme::collect::{colon, comma, ellipsis, newline, parenthesis_close, parenthesis_open, period, unlexable};

lazy_static! {
    static ref SEPARATOR_RE: Regex = Regex::new("^(\\.\\.\\.|…|\\.|,|:|\r\n|\n|\r)").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the lexemes to the Lexer.
pub fn lex_separators(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    let mut found_newline = false;
    while let ReaderResult::Match(sym) = reader.strip_match(&*SEPARATOR_RE) {
        let lexeme = match sym.as_str() {
            r"..." | r"…" => ellipsis(),
            r"." => period(),
            r"," => comma(),
            r":" => colon(),
            "\r\n" | "\n" | "\r" => {
                // Indentation should be parsed after a newline, so stop.
                found_newline = true;
                lexer.set_at_indentable(true);
                newline()
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
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexemes};
    use crate::lexeme::collect::{colon, comma, ellipsis, newline, period, unlexable};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    use super::lex_separators;

    fn check(input: &str, expected: &[Lexemes]) {
        let expected: LexemeCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
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
        check(r"...", &[ellipsis()]);
        check(r"…", &[ellipsis()]);
    }

    #[test]
    fn multiple_ellipsis() {
        check(r"...…", &[ellipsis(), ellipsis()]);
        check(r"…...", &[ellipsis(), ellipsis()]);
        check(r"……", &[ellipsis(), ellipsis()]);
        check(r"......", &[ellipsis(), ellipsis()]);
    }

    #[test]
    fn single_period() {
        check(r".", &[period()]);
    }

    #[test]
    fn multiple_period() {
        check(r"..", &[period(), period()]);
        check(r".. .", &[period(), period(), period()]);
    }

    #[test]
    fn single_comma() {
        check(r",", &[comma()]);
    }

    #[test]
    fn multiple_comma() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r",,", &[comma(), comma()]);
    }

    #[test]
    fn single_colon() {
        check(r":", &[colon()]);
    }

    #[test]
    fn multiple_colon() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r"::", &[colon(), colon()]);
    }

    #[test]
    fn single_newline() {
        check("\r\n", &[newline()]);
        check("\n", &[newline()]);
        check("\r", &[newline()]);
    }

    #[test]
    fn stop_after_newline() {
        check("\r\n:", &[newline()]);
        check("\n...", &[newline()]);
        check("\r,", &[newline()]);
    }

    #[test]
    fn multiple_newline() {
        // Only one newline should be matched
        check("\n\n", &[newline()]);
        check("\r\r", &[newline()]);
        check("\r\n \r\n", &[newline()]);
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
        check(",....\r\n", &[comma(), ellipsis(), period(), newline()]);
    }

    #[test]
    fn combined_2() {
        check("...….,\n,", &[ellipsis(), ellipsis(), period(), comma(), newline()]);
    }

    #[test]
    fn combined_3() {
        check(
            "...:,\n:",
            &[
                ellipsis(),
                colon(),
                comma(),
                newline(),
                // stop after newline
            ],
        );
    }

    #[test]
    fn and_words() {
        check(r"...abc", &[ellipsis()]);
        check(r".abc", &[period()]);
        check(r",abc", &[comma()]);
        check(":abc", &[colon()]);
        check("\nabc", &[newline()]);
    }
}
