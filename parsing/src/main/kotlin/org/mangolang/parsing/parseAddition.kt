package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream

fun parseAddition(tokens: TokenStream): ExpressionAST {
    val lhsMultiplication = parseMultiplication(tokens)
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken && maybeOperator.isAddSub) {
        /* Attempt to parse `Multiplication ("+" | "-") Multiplication`. */
        tokens.take()
        val rhsMultiplication = parseAddition(tokens)
        return ConcreteBinaryOperation(
                lhsMultiplication,
                ConcreteBinaryOperator(maybeOperator),
                rhsMultiplication
        )
    }
    /* Parsing `Multiplication ("+" | "-") Multiplication` failed, just use Multiplication. */
    return lhsMultiplication
}

