use mango::ast_full::special::UnparseableAST;

#[test]
fn test_ast_equality() {
    //    let twinOne = ConcreteBinaryOperation(
    //        IntegerAST(IntegerToken(7)),
    //        ConcreteBinaryOperator(OperatorToken("*")),
    //        NegateOperationAST(IntegerAST(IntegerToken(3))),
    //    )
    //    let twinTwo = ConcreteBinaryOperation(
    //        IntegerAST(IntegerToken(7)),
    //        ConcreteBinaryOperator(OperatorToken("*")),
    //        NegateOperationAST(IntegerAST(IntegerToken(3))),
    //    )
    //    assert_eq!(twinOne, twinTwo)
    //    assert_eq!(twinOne.hashCode(), twinTwo.hashCode())
}

#[test]
fn test_ast_inequality() {
    //    assert_ne!(IntegerAST(IntegerToken(7)), NegateOperationAST(IntegerAST(IntegerToken(7))))
    //    assert_ne!(IntegerAST(IntegerToken(7)), IntegerAST(IntegerToken(8)))
    //    assert_ne!(NegateOperationAST(IntegerAST(IntegerToken(7))), NegateOperationAST(IntegerAST(IntegerToken(8))))
    //    assert_ne!(ConcreteBinaryOperator(OperatorToken("*")), ConcreteBinaryOperator(OperatorToken("/")))
    //    assert_ne!(ConcreteBinaryOperation(IntegerAST(IntegerToken( + 7)), ConcreteBinaryOperator(OperatorToken("*")), IntegerAST(IntegerToken(3))),
    //    ConcreteBinaryOperation(IntegerAST(IntegerToken(-7)), ConcreteBinaryOperator(OperatorToken("*")), IntegerAST(IntegerToken(3)))
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
