use crate::ast_full::util::lex_list::LexList;

pub fn parse_addition(lex: LexList) {
    //TODO @mark:
//    lex
//    val lhsMultiplication = parseMultiplication(listener, tokens)
//    val maybeOperator = tokens.peek()
//    if (maybeOperator is OperatorToken && maybeOperator.isAddSub) {
//        /* Attempt to parse `Multiplication ("+" | "-") Multiplication`. */
//        tokens.take()
//        val rhsMultiplication = parseExpression(listener, tokens)
//        return ConcreteBinaryOperation(
//            lhsMultiplication,
//            ConcreteBinaryOperator(maybeOperator),
//            rhsMultiplication
//        )
//    }
//    /* Parsing `Multiplication ("+" | "-") Multiplication` failed, just use Multiplication. */
//    return lhsMultiplication
}

pub fn parse_multiplication(lex: LexList) {}

pub fn parse_unary(lex: LexList) {}
