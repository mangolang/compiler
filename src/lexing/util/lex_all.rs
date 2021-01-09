use crate::lexing::typ::Lexer;
use crate::lexing::typ::MaybeLexeme;
use crate::lexing::util::lex_list::LexList;

pub fn lex_all(lexer: &mut impl Lexer) -> LexList {
    let mut list = Vec::with_capacity(512);
    while let MaybeLexeme::Lexeme(lexeme) = lexer.lex() {
        list.push(lexeme)
    }
    list.shrink_to_fit();
    LexList::from_lexemes(list)
}
