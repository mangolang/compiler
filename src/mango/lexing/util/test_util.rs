use mango::io::fortest::stringreader::StringReader;
use mango::lexing::combi_lexer::CombiLexer;
use mango::lexing::util::lex_all::lex_all;
use mango::lexing::util::lex_all::LexList;
use mango::token::Tokens;
use mango::util::encdec::to_text::ToText;

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
