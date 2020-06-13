use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{parenthesis_close, parenthesis_open, comma, ellipsis, period, newline, unlexable};

lazy_static! {
    static ref SEPARATOR_RE: Regex = Regex::new("^(\\.\\.\\.|…|\\.|,|\r\n|\n|\r)").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the tokens to the Lexer.
pub fn lex_separators(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    let mut found_newline = false;
    while let ReaderResult::Match(sym) = reader.strip_match(&*SEPARATOR_RE) {
        lexer.add(match sym.as_str() {
            r"..." | r"…" => ellipsis(),
            r"." => period(),
            r"," => comma(),
            "\r\n" | "\n" | "\r" => {
                // Indentation should be parsed after a newline, so stop.
                found_newline = true;
                newline()
            },
            _ => unreachable!(),
        });
        if found_newline { break }
    }
}

#[cfg(test)]
mod grouping {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};
    use crate::token::collect::{comma, ellipsis, period, newline, unlexable};

    use super::lex_separators;
    use crate::token::collect::token_list::TokenList;

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
    fn single_newline() {
        check("\r\n", &vec![newline()]);
        check("\n", &vec![newline()]);
        check("\r", &vec![newline()]);
    }

    #[test]
    fn multiple_newline() {
        // Only one newline should be matched
        check("\n\n", &vec![newline()]);
        check("\r\r", &vec![newline()]);
        check("\r\n \r\n", &vec![newline()]);
    }

    #[test]
    fn combined_1() {
        check(",....\r\n", &vec![
            comma(),
            ellipsis(),
            period(),
            newline(),
        ]);
    }

    #[test]
    fn combined_2() {
        check("...….,\n,", &vec![
            ellipsis(),
            ellipsis(),
            period(),
            comma(),
            newline(),
        ]);
    }

    #[test]
    fn and_words() {
        check(r"...abc", &vec![ellipsis()]);
        check(r".abc", &vec![period()]);
        check(r",abc", &vec![comma()]);
        check("\nabc", &vec![newline()]);
    }
}
