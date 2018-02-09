package org.mangolang.token

import org.mangolang.util.escape.toStringLiteral

/**
 * Token for a text literal.
 */
data class StringToken(public val value: String): Token {
    override fun asText(): CharSequence = toStringLiteral(value)
}


