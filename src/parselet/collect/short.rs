use ::std::str::FromStr;

// use crate::common::error::{ErrMsg, MsgResult};
// use crate::util::codeparts::Keyword;
// use crate::util::numtype::f64eq;
// use crate::parselet::{Parselets, UnparseableParselet, ExpressionParselets, LiteralParselet};
use crate::lexeme::{Lexemes, LiteralLexeme, OperatorLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets, LiteralParselet, Parselets, UnparseableParselet};

// pub fn association(txt: &str) -> MsgResult<Parselets> {
//     Ok(Parselets::Association(AssociationParselet::from_str(txt)?))
// }
//
// pub fn identifier(txt: &str) -> MsgResult<Parselets> {
//     Ok(Parselets::Identifier(IdentifierParselet::from_str(txt)?))
// }
//
// /// Parse a keyword, including reserved keywords for future use.
// pub fn keyword_or_reserved(txt: &str) -> MsgResult<Parselets> {
//     Ok(Parselets::Keyword(KeywordParselet::from_str(txt)?))
// }
//
// /// Parse a keyword, but fail if it a reserved keyword, rather than one that already works.
// pub fn keyword_supported(txt: &str) -> MsgResult<Parselets> {
//     match Keyword::from_str(txt)? {
//         Keyword::Reserved(word) => Err(ErrMsg::new(format!("Keyword '{}' not implemented", word))),
//         kw => Ok(Parselets::Keyword(KeywordParselet::from_keyword(kw))),
//     }
// }

pub fn literal(lexeme: LiteralLexeme) -> ExpressionParselets {
    ExpressionParselets::Literal(LiteralParselet::new(lexeme))
}

pub fn binary(left: ExpressionParselets, operator: OperatorLexeme, right: ExpressionParselets) -> ExpressionParselets {
    ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(left, operator, right))
}

// pub fn operator(txt: &str) -> MsgResult<Parselets> {
//     Ok(Parselets::Operator(OperatorParselet::from_str(&txt)?))
// }

// pub fn parenthesis_open() -> Parselets {
//     Parselets::ParenthesisOpen(ParenthesisOpenParselet::new())
// }
//
// pub fn parenthesis_close() -> Parselets {
//     Parselets::ParenthesisClose(ParenthesisCloseParselet::new())
// }
//
// pub fn bracket_open() -> Parselets {
//     Parselets::BracketOpen(BracketOpenParselet::new())
// }
//
// pub fn bracket_close() -> Parselets {
//     Parselets::BracketClose(BracketCloseParselet::new())
// }

// pub fn end_statement() -> Parselets {
//     //TODO @mark: for now only create newlines
//     Parselets::EndStatement(EndStatementParselet::new_end_line())
// }

// pub fn start_block() -> Parselets {
//     Parselets::StartBlock(StartBlockParselet::new())
// }
//
// pub fn end_block() -> Parselets {
//     Parselets::EndBlock(EndBlockParselet::new2())
// }
//
// pub fn colon() -> Parselets {
//     Parselets::Colon(ColonParselet::new())
// }
// pub fn comma() -> Parselets {
//     Parselets::Comma(CommaParselet::new())
// }
// pub fn ellipsis() -> Parselets {
//     Parselets::Ellipsis(EllipsisParselet::new())
// }
// pub fn period() -> Parselets {
//     Parselets::Period(PeriodParselet::new())
// }
// pub fn newline() -> Parselets {
//     Parselets::Newline(NewlineParselet::new())
// }

pub fn unparseable(lexemes: Vec<Lexemes>) -> Parselets {
    Parselets::Unparseable(UnparseableParselet::from_lexemes(lexemes))
}
