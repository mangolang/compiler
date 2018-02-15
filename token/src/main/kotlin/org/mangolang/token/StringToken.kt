
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

import org.mangolang.util.escape.toStringLiteral

/**
 * Token for a text literal.
 */
data class StringToken(public val value: String) : Token {
    override fun asText(): CharSequence = toStringLiteral(value)
}
