use std::str::FromStr;

use crate::parselet::FullAST;
use crate::parselet::node::AssignmentAST;
use crate::parselet::node::BinaryOperationAST;
use crate::parselet::node::UnaryOperationAST;
use crate::parselet::operator::BinOpSymbol;
use crate::parselet::special::UnparseableAST;
use crate::parselet::terminal::FloatLiteralAST;
use crate::parselet::terminal::IntLiteralAST;
use crate::parselet::terminal::LiteralAST;
use crate::parselet::terminal::OperatorAST;
use crate::parselet::terminal::StringLiteralAST;
use crate::parselet::terminal::VariableAST;
use crate::token::Tokens;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::util::strtype::Name;
use crate::util::strtype::StrType;

#[test]
fn test_nested_ast_eq() {
    let twin_one = BinaryOperationAST::new(
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        OperatorAST::from_symbol(BinOpSymbol::Plus),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(BinOpSymbol::Plus),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        )),
    );
    let twin_two = BinaryOperationAST::new(
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        OperatorAST::from_symbol(BinOpSymbol::Plus),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(BinOpSymbol::Plus),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        )),
    );
    assert_eq!(twin_one, twin_two);
    // todo: hash not implemented, so not tested yet
    // assert_eq!(calculate_hash(&twin_one), calculate_hash(&twin_two));
}

#[test]
fn test_simple_ast_eq_ne() {
    let nodes = vec![
        FullAST::Operator(OperatorAST::from_symbol(BinOpSymbol::Plus)),
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(1))),
        FullAST::Literal(LiteralAST::Float(FloatLiteralAST::new(1.))),
        FullAST::Literal(LiteralAST::String(StringLiteralAST::new("1".to_string()))),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(BinOpSymbol::Dash),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(1))),
        )),
        FullAST::BinaryOperation(BinaryOperationAST::new(
            FullAST::Literal(LiteralAST::Float(FloatLiteralAST::new(1.))),
            OperatorAST::from_symbol(BinOpSymbol::Plus),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(1))),
        )),
        FullAST::Variable(VariableAST::new(Name::from_valid("my_var"))),
        FullAST::Assignment(AssignmentAST::new(
            VariableAST::new(Name::from_valid("my_var")),
            FullAST::Literal(LiteralAST::String(StringLiteralAST::new("1".to_string()))),
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
    let unp: UnparseableAST;
    unp = UnparseableAST::from_tokens(vec![
        Tokens::Identifier(IdentifierToken::from_str("x").unwrap()),
        Tokens::Operator(OperatorToken::from_str("<").unwrap()),
        Tokens::Literal(LiteralToken::Int(128)),
    ]);
    assert_eq!(unp, unp);
    let unp2 = UnparseableAST::from_tokens(vec![
        Tokens::Identifier(IdentifierToken::from_str("y").unwrap()),
        Tokens::Operator(OperatorToken::from_str("<").unwrap()),
        Tokens::Literal(LiteralToken::Int(128)),
    ]);
    assert_ne!(unp, unp2);
}
