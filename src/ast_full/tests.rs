use crate::ast_full::node::AssignmentAST;
use crate::ast_full::node::BinaryOperationAST;
use crate::ast_full::node::UnaryOperationAST;
use crate::ast_full::special::UnparseableAST;
use crate::ast_full::terminal::FloatLiteralAST;
use crate::ast_full::terminal::IntLiteralAST;
use crate::ast_full::terminal::LiteralAST;
use crate::ast_full::terminal::OperatorAST;
use crate::ast_full::terminal::StringLiteralAST;
use crate::ast_full::terminal::VariableAST;
use crate::ast_full::FullAST;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::token::Tokens;
use crate::util::strtype::Name;
use crate::util::strtype::StrType;
use crate::ast_full::operator::BinOpSymbol;

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
        Box::new(Tokens::Identifier(IdentifierToken::from_str("x".to_owned()).unwrap())),
        Box::new(Tokens::Operator(OperatorToken::from_str("<").unwrap())),
        Box::new(Tokens::Literal(LiteralToken::Int(128))),
    ]);
    assert_eq!(unp, unp);
    let unp2 = UnparseableAST::from_tokens(vec![
        Box::new(Tokens::Identifier(IdentifierToken::from_str("y".to_owned()).unwrap())),
        Box::new(Tokens::Operator(OperatorToken::from_str("<").unwrap())),
        Box::new(Tokens::Literal(LiteralToken::Int(128))),
    ]);
    assert_ne!(unp, unp2);
}
