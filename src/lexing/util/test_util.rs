use crate::io::fortest::stringreader::StringReader;
use crate::lexing::combi_lexer::CombiLexer;
use crate::lexing::util::lex_all::lex_all;
use crate::token::Tokens;
use crate::util::encdec::to_text::ToText;
use crate::ast_full::util::lex_list::LexList;

#[allow(dead_code)]
pub fn assert_text_to_tokens(text: &str, tokens: Vec<Tokens>) {
    let expected = LexList::from_tokens(tokens);
    let actual = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(text.to_owned()))));
    assert_eq!(
        expected,
        actual,
        "\nexpected:\n{}\nactual:\n{}",
        expected.to_text(),
        actual.to_text(),
    );
}
