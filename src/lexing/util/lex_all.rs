use crate::lexing::typ::Lexer;
use crate::lexing::typ::MaybeToken;
use crate::token::Tokens;
use crate::util::encdec::ToText;
use crate::ast_full::util::lex_list::LexList;

pub fn lex_all(lexer: &mut Lexer) -> LexList {
    let mut list = Vec::with_capacity(512);
    while let MaybeToken::Token(token) = lexer.lex() {
        list.push(token)
    }
    list.shrink_to_fit();
    LexList::from_tokens(list)
}
