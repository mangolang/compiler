use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::collect::{colon, comma, ellipsis, newline, parenthesis_close, parenthesis_open, period, unlexable};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};

lazy_static! {
    static ref SEPARATOR_RE: Regex = Regex::new("^(\\.\\.\\.|…|\\.|,|:|\r\n|\n|\r)").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the tokens to the Lexer.
pub fn lex_separators(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    let mut found_newline = false;
    while let ReaderResult::Match(sym) = reader.strip_match(&*SEPARATOR_RE) {
        let token = match sym.as_str() {
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
        lexer.add(token);
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
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{colon, comma, ellipsis, newline, period, unlexable};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::lex_separators;

    fn check(input: &str, expected: &[Tokens]) {
        let expected: TokenList = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_separators(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected);
    }

    #[test]
    fn empty() {
        check("", &vec![]);
        check("    \t", &vec![]);
    }

    #[test]
    fn mismatch() {
        check("(", &vec![]);
        check("0", &vec![]);
        check("a", &vec![]);
    }

    #[test]
    fn single_ellipsis() {
        check(r"...", &vec![ellipsis()]);
        check(r"…", &vec![ellipsis()]);
    }

    #[test]
    fn multiple_ellipsis() {
        check(r"...…", &vec![ellipsis(), ellipsis()]);
        check(r"…...", &vec![ellipsis(), ellipsis()]);
        check(r"……", &vec![ellipsis(), ellipsis()]);
        check(r"......", &vec![ellipsis(), ellipsis()]);
    }

    #[test]
    fn single_period() {
        check(r".", &vec![period()]);
    }

    #[test]
    fn multiple_period() {
        check(r"..", &vec![period(), period()]);
        check(r".. .", &vec![period(), period(), period()]);
    }

    #[test]
    fn single_comma() {
        check(r",", &vec![comma()]);
    }

    #[test]
    fn multiple_comma() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r",,", &vec![comma(), comma()]);
    }

    #[test]
    fn single_colon() {
        check(r":", &vec![colon()]);
    }

    #[test]
    fn multiple_colon() {
        // Probably a syntax error later on, but that's the parser's problem.
        check(r"::", &vec![colon(), colon()]);
    }

    #[test]
    fn single_newline() {
        check("\r\n", &vec![newline()]);
        check("\n", &vec![newline()]);
        check("\r", &vec![newline()]);
    }

    #[test]
    fn stop_after_newline() {
        check("\r\n:", &vec![newline()]);
        check("\n...", &vec![newline()]);
        check("\r,", &vec![newline()]);
    }

    #[test]
    fn multiple_newline() {
        // Only one newline should be matched
        check("\n\n", &vec![newline()]);
        check("\r\r", &vec![newline()]);
        check("\r\n \r\n", &vec![newline()]);
    }

    #[test]
    fn after_mismatch() {
        check(r"1 …", &vec![]);
        check(r"1 .", &vec![]);
        check(r"1 ,", &vec![]);
        check("1 \r", &vec![]);
    }

    #[test]
    fn combined_1() {
        check(",....\r\n", &vec![comma(), ellipsis(), period(), newline()]);
    }

    #[test]
    fn combined_2() {
        check("...….,\n,", &vec![ellipsis(), ellipsis(), period(), comma(), newline()]);
    }

    #[test]
    fn combined_3() {
        check(
            "...:,\n:",
            &vec![
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
        check(r"...abc", &vec![ellipsis()]);
        check(r".abc", &vec![period()]);
        check(r",abc", &vec![comma()]);
        check(":abc", &vec![colon()]);
        check("\nabc", &vec![newline()]);
    }
}
