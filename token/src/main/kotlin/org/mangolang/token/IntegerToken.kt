package org.mangolang.token

/**
 * Token for an integer literal.
 */
data class IntegerToken(public val value: Int): Token {
    override fun asText(): CharSequence = value.toString()
}

