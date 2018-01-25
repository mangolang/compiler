package org.mangolang.parsing

import org.mangolang.fullast.LiteralAST
import org.mangolang.fullast.IntegerAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.TokenStream

fun parseLiteral(tokens: TokenStream): LiteralAST {
    val token = tokens.take()
    return when (token) {
        is IntegerToken -> IntegerAST(token)
        null -> TODO("Expected a literal, but the stream ended")
        else -> TODO("Expected a literal, but found ____")
    }
}


