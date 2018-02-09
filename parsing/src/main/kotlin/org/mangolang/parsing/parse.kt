package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener
import org.mangolang.util.text.Message

/**
 * This is the entry point for the recursive descent parser. It will
 * try to match a pattern in the token stream, producing nested
 * abstract syntax tree nodes.
 */
fun parse(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    val expression = parseExpression(listener, tokens)
    if (tokens.peek() != null) {
        listener.error(SyntaxError(Message("Parsing stopped before the end of input."), null))
    }
    return expression
}

