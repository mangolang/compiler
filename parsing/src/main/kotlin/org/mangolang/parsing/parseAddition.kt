package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener

/**
 * Parse an addition or subtraction expression (left + right or left - right).
 */
fun parseAddition(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val lhsMultiplication = parseMultiplication(listener, tokens)
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken && maybeOperator.isAddSub) {
        /* Attempt to parse `Multiplication ("+" | "-") Multiplication`. */
        tokens.take()
        val rhsMultiplication = parseAddition(listener, tokens)
        return ConcreteBinaryOperation(
                lhsMultiplication,
                ConcreteBinaryOperator(maybeOperator),
                rhsMultiplication
        )
    }
    /* Parsing `Multiplication ("+" | "-") Multiplication` failed, just use Multiplication. */
    return lhsMultiplication
}

