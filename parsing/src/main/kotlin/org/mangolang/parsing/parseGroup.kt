package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.ParenthesisCloseToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.TokenStream

fun parseGroup(tokens: TokenStream): ExpressionAST {
    val token = tokens.peek()
    if (token is ParenthesisOpenToken) {
        tokens.take()
        val expr = parseGroup(tokens)
        val end = tokens.take()
        if (end !is ParenthesisCloseToken) {
            TODO("Expected end of parenthesized block, but found ${end}")
        }
        return expr
    }
    return parseExpression(tokens)
}


