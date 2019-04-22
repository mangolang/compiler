use mango::ast_full::node::AssignmentAST;
use mango::ast_full::node::BinaryOperationAST;
use mango::ast_full::node::UnaryOperationAST;
use mango::ast_full::special::UnparseableAST;
use mango::ast_full::terminal::FloatLiteralAST;
use mango::ast_full::terminal::IntLiteralAST;
use mango::ast_full::terminal::LiteralAST;
use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::terminal::StringLiteralAST;
use mango::ast_full::terminal::VariableAST;
use mango::ast_full::FullAST;
use mango::token::tokens::IdentifierToken;
use mango::token::tokens::LiteralToken;
use mango::token::tokens::OperatorToken;
use mango::token::Tokens;
use mango::util::codeparts::Symbol;
use mango::util::strtype::Name;
use mango::util::strtype::StrType;

#[test]
fn test_nested_ast_eq() {
    let twin_one = BinaryOperationAST::new(
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        OperatorAST::from_symbol(Symbol::Plus),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(Symbol::Plus),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        )),
    );
    let twin_two = BinaryOperationAST::new(
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(7))),
        OperatorAST::from_symbol(Symbol::Plus),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(Symbol::Plus),
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
        FullAST::Operator(OperatorAST::from_symbol(Symbol::Plus)),
        FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(1))),
        FullAST::Literal(LiteralAST::Float(FloatLiteralAST::new(1.))),
        FullAST::Literal(LiteralAST::String(StringLiteralAST::new("1".to_string()))),
        FullAST::UnaryOperation(UnaryOperationAST::new(
            OperatorAST::from_symbol(Symbol::Dash),
            FullAST::Literal(LiteralAST::Int(IntLiteralAST::new(1))),
        )),
        FullAST::BinaryOperation(BinaryOperationAST::new(
            FullAST::Literal(LiteralAST::Float(FloatLiteralAST::new(1.))),
            OperatorAST::from_symbol(Symbol::Plus),
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
