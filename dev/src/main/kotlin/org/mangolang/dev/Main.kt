package org.mangolang.dev

import org.mangolang.parsing.parse
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.ParenthesisCloseToken
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.token.mock.FixedTokenStream

fun main(arg: Array<String>) {
    // For development testing, bypass the command line interface and tell the conductor to do things.
    println("Hello, world!")
    val tokens: TokenStream = FixedTokenStream(listOf(
            ParenthesisOpenToken,
            IntegerToken(5),
            OperatorToken("+"),
            ParenthesisOpenToken,
            IntegerToken(3),
            OperatorToken("+"),
            IntegerToken(2),
            ParenthesisCloseToken,
            OperatorToken("-"),
            IntegerToken(6),
            ParenthesisCloseToken
    ))
    println(tokens)
    println(parse(tokens).asText())
}


