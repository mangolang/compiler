package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream

fun parseMultiplication(tokens: TokenStream): ExpressionAST {
    val lhsUnary = parseUnary(tokens)
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken && maybeOperator.isMultDiv) {
        /* Attempt to parse `UnaryOperation ("*" | "/") UnaryOperation`. */
        tokens.take()
        val rhsUnary = parseMultiplication(tokens)
        return ConcreteBinaryOperation(
                lhsUnary,
                ConcreteBinaryOperator(maybeOperator),
                rhsUnary
        )
    }
    /* Parsing `UnaryOperation ("*" | "/") UnaryOperation` failed, just use UnaryOperation. */
    return lhsUnary
}

