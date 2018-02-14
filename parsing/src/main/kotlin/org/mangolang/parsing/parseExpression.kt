package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.UnparseableAST
import org.mangolang.token.ParenthesisCloseToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener
import org.mangolang.util.text.Message

/**
 * Parse an expression (by delegating to other parse functions).
 */
fun parseExpression(listener: ProblemListener, tokens: TokenStream): ExpressionAST =
        parseAddition(listener, tokens)

/**
 * Parse a grouped expression (there no special AST element for
 * grouping so parenthesis information is list at this point).
 */
fun parseGroupedExpression(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val token = tokens.take()
    if (token !is ParenthesisOpenToken) {
        /* It is really necessary to stop here to avoid infinite recursion. */
        listener.error(SyntaxError(Message("Expected a grouped expression, but found ${tokens.peek()}."),
                null))
        return UnparseableAST(tokens.peek())
    }
    val expression = parseExpression(listener, tokens)
    if (tokens.peek() !is ParenthesisCloseToken) {
        listener.error(SyntaxError(Message("Did not find a closing parenthesis at end of expression; " +
                "found ${tokens.peek()}."), null))
        return UnparseableAST(tokens.peek())
    }
    tokens.take()
    return expression
}
