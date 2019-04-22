use crate::ast_full::node::BinaryOperationAST;
use crate::ast_full::node::UnaryOperationAST;
use crate::ast_full::special::UnparseableAST;
use crate::ast_full::terminal::IntLiteralAST;
use crate::ast_full::terminal::OperatorAST;
use crate::ast_full::terminal::Symbol;

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
    // todo: hash not implemented, so not tested yet
    // assert_eq!(calculate_hash(&twin_one), calculate_hash(&twin_two));
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
