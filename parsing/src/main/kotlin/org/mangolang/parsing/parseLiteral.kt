package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.LiteralAST
import org.mangolang.fullast.IntegerAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.TokenStream

fun parseLiteral(tokens: TokenStream): ExpressionAST {
    val token = tokens.peek()
    return when (token) {
        is IntegerToken -> {
            tokens.take()
            IntegerAST(token)
        }
        is ParenthesisOpenToken -> {
            parseGroupedExpression(tokens)
        }
        null -> TODO("Expected a literal, but the stream ended")
        else -> TODO("Expected a literal, but found ____")
    }
}

