
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.UnparseableAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.TokenStream
import org.mangolang.util.text.Message
import org.mangolang.util.errors.ProblemListener

/**
 * Parse a literal value expression.
 */
fun parseLiteral(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val token = tokens.peek()
    return when (token) {
        is IntegerToken -> {
            tokens.take()
            IntegerAST(token)
        }
        is ParenthesisOpenToken -> parseGroupedExpression(listener, tokens)
        null -> {
            listener.error(SyntaxError(Message("Expected a literal, but the stream ended."), null))
            UnparseableAST()
        }
        else -> {
            listener.error(SyntaxError(Message("Expected a literal, but found $token."), token))
            UnparseableAST(token)
        }
    }
}
