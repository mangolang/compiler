use mango::ast_full::special::UnparseableAST;
use mango::ast_full::terminal::IntLiteralAST;
use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::terminal::Symbol;
use mango::ast_full::node::BinaryOperationAST;
use mango::ast_full::node::UnaryOperationAST;

#[test]
fn test_ast_equality() {
    let twin_one = BinaryOperationAST::new(
        Box::new(IntLiteralAST::new(7)),
        OperatorAST::from_symbol(Symbol::Plus),
        Box::new(UnaryOperationAST::new(
            OperatorAST::from_symbol(Symbol::Plus),
            Box::new(IntLiteralAST::new(3)),
        )),
    );
    let twin_two = BinaryOperationAST::new(
        Box::new(IntLiteralAST::new(7)),
        OperatorAST::from_symbol(Symbol::Plus),
        Box::new(UnaryOperationAST::new(
            OperatorAST::from_symbol(Symbol::Plus),
            Box::new(IntLiteralAST::new(3)),
        )),
    );
    assert_eq!(twin_one, twin_two);
    //    assert_eq!(twin_one.hashCode(), twin_two.hashCode());
}

#[test]
fn test_ast_inequality() {
    //    assert_ne!(IntLiteralAST(IntegerToken(7)), UnaryOperationAST(IntLiteralAST(IntegerToken(7))))
    //    assert_ne!(IntLiteralAST(IntegerToken(7)), IntLiteralAST(IntegerToken(8)))
    //    assert_ne!(UnaryOperationAST(IntLiteralAST(IntegerToken(7))), UnaryOperationAST(IntLiteralAST(IntegerToken(8))))
    //    assert_ne!(ConcreteBinaryOperator(OperatorAST("*")), ConcreteBinaryOperator(OperatorAST("/")))
    //    assert_ne!(BinaryOperationAST(IntLiteralAST(IntegerToken( + 7)), ConcreteBinaryOperator(OperatorAST("*")), IntLiteralAST(IntegerToken(3))),
    //    BinaryOperationAST(IntLiteralAST(IntegerToken(-7)), ConcreteBinaryOperator(OperatorAST("*")), IntLiteralAST(IntegerToken(3)))
    //    )
}

#[test]
fn test_unparseable_equality() {
    let unp: UnparseableAST;
    unp = UnparseableAST::from_tokens(vec![]);
    assert_eq!(unp, unp);
    //        let unp = UnparseableAST::from_tokens(vec![IntegerToken()]);
    //        assert_eq!(up, up)
    //        assert_ne!(UnparseableAST(null), UnparseableAST(null))
    //        assert_ne!(UnparseableAST(IntegerToken(7)), UnparseableAST(IntegerToken(7)))
}
