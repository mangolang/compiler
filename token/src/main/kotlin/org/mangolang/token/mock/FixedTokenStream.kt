package org.mangolang.token.mock

import org.mangolang.token.Token
import org.mangolang.token.TokenStream
import org.mangolang.util.HASH_CODE_MULT

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

    override fun toString(): String = tokens.map { it.asText() }.joinToString("")

    /**
     * This checks whether the tokens in the stream are equal. The current reading index is not taken into account.
     */
    @Suppress("EqualsAlwaysReturnsTrueOrFalse")  // LATER: this suppression should not be necessary...
    override fun equals(other: Any?): Boolean {
        // LATER: check the exact class instead of subclass?
        if (this === other) {
            return true
        }
        if (other !is FixedTokenStream) {
            return false
        }
        if (tokens.size != other.tokens.size) {
            return false
        }
        for (k in 0 until tokens.size) {
            if (tokens[k] != other.tokens[k]) {
                return false
            }
        }
        return true
    }

    /**
     * The current reading index is not taken into account for the hashcode.
     *
     * @implNote This relies on silent integer overflow.
     */
    override fun hashCode(): Int {
        var hash = this::class.hashCode()
        for (token in tokens) {
            hash *= HASH_CODE_MULT
            hash += token.hashCode()
        }
        return hash
    }
}

