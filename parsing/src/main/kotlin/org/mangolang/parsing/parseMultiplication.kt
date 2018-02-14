
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener

/**
 * Parse a multiplication or division expression (left * right or left / right).
 */
fun parseMultiplication(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val lhsUnary = parseUnary(listener, tokens)
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken && maybeOperator.isMultDiv) {
        /* Attempt to parse `UnaryOperation ("*" | "/") UnaryOperation`. */
        tokens.take()
        val rhsUnary = parseMultiplication(listener, tokens)
        return ConcreteBinaryOperation(
                lhsUnary,
                ConcreteBinaryOperator(maybeOperator),
                rhsUnary
        )
    }
    /* Parsing `UnaryOperation ("*" | "/") UnaryOperation` failed, just use UnaryOperation. */
    return lhsUnary
}
