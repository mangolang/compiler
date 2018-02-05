package org.mangolang.token.mock

import org.mangolang.token.Token
import org.mangolang.token.TokenStream

/**
 * A stream that just produces the tokens it received as input.
 */
class FixedTokenStream(private val tokens: List<Token>): TokenStream {
    private var index = 0

    override fun peek(): Token? {
        if (index >= tokens.size) {
            return null
        }
        return tokens[index]
    }

    override fun take(): Token? {
        val result = peek()
        index++
        return result
    }

    override fun toString(): String {
        return tokens.map { it.asText() }.joinToString("")
    }
}

