package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.NegateOperationAST
import org.mangolang.fullast.UnparseableAST
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener
import org.mangolang.util.text.Message

/**
 * Parse a unary operation. Currently +x or -x for literals x.
 */
fun parseUnary(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val maybeOperator = tokens.peek()
    if (maybeOperator is OperatorToken) {
        /* Attempt to parse `("+" | "-") UnaryOperation`. */
        tokens.take()
        val subject = parseUnary(listener, tokens)
        if (maybeOperator.isNegate) {
            return NegateOperationAST(subject)
        } else if (maybeOperator.isUnaryNoop) {
            return subject
        } else {
            listener.error(SyntaxError(Message("Found token $maybeOperator but cannot parse an expression " +
                    "starting with it."), null))
            return UnparseableAST(maybeOperator)
        }
    }
    /* No unary operation, parse literal instead. */
    return parseLiteral(listener, tokens)
}

