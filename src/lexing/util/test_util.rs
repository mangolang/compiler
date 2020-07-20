use crate::io::fortest::stringreader::StringReader;
use crate::lexing::combi_lexer::CombiLexer;
use crate::lexing::util::lex_all::lex_all;
use crate::lexing::util::lex_list::LexList;
use crate::lexeme::Lexemes;
use crate::util::encdec::to_text::ToText;

#[allow(dead_code)]
pub fn assert_text_to_lexemes(text: &str, lexemes: Vec<Lexemes>) {
    let expected = LexList::from_lexemes(lexemes);
    let actual = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(text.to_owned()))));
    assert_eq!(
        expected,
        actual,
        "\nexpected:\n{}\nactual:\n{}",
        expected.to_text(),
        actual.to_text(),
    );
}
