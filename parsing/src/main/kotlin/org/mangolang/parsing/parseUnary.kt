package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.NegateOperationAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream

fun parseUnary(tokens: TokenStream): ExpressionAST {
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken) {
        /* Attempt to parse `("+" | "-") UnaryOperation`. */
        tokens.take()
        val subject = parseUnary(tokens)
        if (maybeOperator.isNegate) {
            return NegateOperationAST(subject)
        } else if (maybeOperator.isUnaryNoop) {
            return subject
        } else {
            TODO("Found token $maybeOperator but cannot parse an expression starting with it")
        }
    }
    /* No unary operation, parse literal instead. */
    return parseLiteral(tokens)
}

