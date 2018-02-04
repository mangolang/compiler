package org.mangolang.parsing

import org.mangolang.fullast.BinaryAST
import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream

fun parseMultiplication(tokens: TokenStream): BinaryAST {
    val lhsUnary = parseUnary(tokens)
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken && maybeOperator.isMultDiv) {
        /* Attempt to parse `UnaryOperation ("*" | "/") UnaryOperation`. */
        tokens.take()
        val rhsUnary = parseUnary(tokens)
        return ConcreteBinaryOperation(
                lhsUnary,
                ConcreteBinaryOperator(maybeOperator),
                rhsUnary
        )
    }
    /* Parsing `UnaryOperation ("*" | "/") UnaryOperation` failed, just use UnaryOperation. */
    return lhsUnary
//    val product = parseMultiplication(tokens)
//    if (product != null) {
//        return product
//    }
//    val next = tokens.take()
//    if (next is OperatorToken && next.symbol == "+") {
//        return parseMultiplication(tokens)
//    }
//    if (next is OperatorToken && next.symbol == "-") {
//        return NegateOperationAST(parseMultiplication(next))
//    }
//    TODO("expect a sum or product, but found ${next}")
}

// sum =  product | sum "+" product | sum "-" product ;
// product = term | product "*" term | product "/" term ;
// term = "-" term | "(" sum ")" | number ;


