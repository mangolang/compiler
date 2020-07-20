use std::str::FromStr;

use crate::parselet::FullParselet;
use crate::parselet::node::AssignmentParselet;
use crate::parselet::node::BinaryOperationParselet;
use crate::parselet::node::UnaryOperationParselet;
use crate::parselet::operator::BinOpSymbol;
use crate::parselet::special::UnparseableParselet;
use crate::parselet::terminal::FloatLiteralParselet;
use crate::parselet::terminal::IntLiteralParselet;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::terminal::OperatorParselet;
use crate::parselet::terminal::StringLiteralParselet;
use crate::parselet::terminal::VariableParselet;
use crate::lexeme::Lexemes;
use crate::lexeme::lexemes::IdentifierLexeme;
use crate::lexeme::lexemes::LiteralLexeme;
use crate::lexeme::lexemes::OperatorLexeme;
use crate::util::strtype::Name;
use crate::util::strtype::StrType;

#[test]
fn test_nested_ast_eq() {
    let twin_one = BinaryOperationParselet::new(
        FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
        OperatorParselet::from_symbol(BinOpSymbol::Plus),
        FullParselet::UnaryOperation(UnaryOperationParselet::new(
            OperatorParselet::from_symbol(BinOpSymbol::Plus),
            FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
        )),
    );
    let twin_two = BinaryOperationParselet::new(
        FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
        OperatorParselet::from_symbol(BinOpSymbol::Plus),
        FullParselet::UnaryOperation(UnaryOperationParselet::new(
            OperatorParselet::from_symbol(BinOpSymbol::Plus),
            FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
        )),
    );
    assert_eq!(twin_one, twin_two);
    // todo: hash not implemented, so not tested yet
    // assert_eq!(calculate_hash(&twin_one), calculate_hash(&twin_two));
}

#[test]
fn test_simple_ast_eq_ne() {
    let nodes = vec![
        FullParselet::Operator(OperatorParselet::from_symbol(BinOpSymbol::Plus)),
        FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
        FullParselet::Literal(LiteralParselet::Float(FloatLiteralParselet::new(1.))),
        FullParselet::Literal(LiteralParselet::String(StringLiteralParselet::new("1".to_string()))),
        FullParselet::UnaryOperation(UnaryOperationParselet::new(
            OperatorParselet::from_symbol(BinOpSymbol::Dash),
            FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
        )),
        FullParselet::BinaryOperation(BinaryOperationParselet::new(
            FullParselet::Literal(LiteralParselet::Float(FloatLiteralParselet::new(1.))),
            OperatorParselet::from_symbol(BinOpSymbol::Plus),
            FullParselet::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
        )),
        FullParselet::Variable(VariableParselet::new(Name::from_valid("my_var"))),
        FullParselet::Assignment(AssignmentParselet::new(
            VariableParselet::new(Name::from_valid("my_var")),
            FullParselet::Literal(LiteralParselet::String(StringLiteralParselet::new("1".to_string()))),
        )),
    ];
    for (i, left) in nodes.iter().enumerate() {
        for (j, right) in nodes.iter().enumerate() {
            if i == j {
                assert_eq!(left, right);
            } else {
                assert_ne!(left, right);
            }
        }
    }
}

#[test]
fn test_unparseable_equality() {
    let unp: UnparseableParselet;
    unp = UnparseableParselet::from_lexemes(vec![
        Lexemes::Identifier(IdentifierLexeme::from_str("x").unwrap()),
        Lexemes::Operator(OperatorLexeme::from_str("<").unwrap()),
        Lexemes::Literal(LiteralLexeme::Int(128)),
    ]);
    assert_eq!(unp, unp);
    let unp2 = UnparseableParselet::from_lexemes(vec![
        Lexemes::Identifier(IdentifierLexeme::from_str("y").unwrap()),
        Lexemes::Operator(OperatorLexeme::from_str("<").unwrap()),
        Lexemes::Literal(LiteralLexeme::Int(128)),
    ]);
    assert_ne!(unp, unp2);
}
