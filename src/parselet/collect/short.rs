use crate::lexeme::{FQIdentifierLexeme, Lexeme, LiteralLexeme, OperatorLexeme};
use crate::parselet::node::binary_operation::BinaryOperationParselet;
use crate::parselet::node::function_call::{ExprGroup, FunctionCallParselet};
use crate::parselet::special::UnparseableParselet;
use crate::parselet::terminal::{ArrayLiteralParselet, LiteralParselet, VariableParselet};
use crate::parselet::{ExpressionParselets, Parselets};

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

pub fn variable(lexeme: FQIdentifierLexeme) -> ExpressionParselets {
    ExpressionParselets::Variable(VariableParselet::new(lexeme))
}

pub fn literal(lexeme: LiteralLexeme) -> ExpressionParselets {
    ExpressionParselets::Literal(LiteralParselet::new(lexeme))
}

pub fn binary(left: ExpressionParselets, operator: OperatorLexeme, right: ExpressionParselets) -> ExpressionParselets {
    ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(left, operator, right))
}

pub fn function_call(function: ExpressionParselets, args: ExprGroup) -> ExpressionParselets {
    ExpressionParselets::Call(FunctionCallParselet::new(function, args))
}

pub fn array_index(array: ExpressionParselets, indices: ExprGroup) -> ExpressionParselets {
    ExpressionParselets::Call(FunctionCallParselet::new(array, indices))
}

pub fn array_literal(content: Vec<ExpressionParselets>) -> ExpressionParselets {
    ExpressionParselets::ArrayLiteral(ArrayLiteralParselet::new(content))
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

pub fn unparseable(lexemes: Vec<Lexeme>) -> Parselets {
    Parselets::Unparseable(UnparseableParselet::from_lexemes(lexemes))
}
