
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * Closing parenthesis: ``)``.
 */
class ParenthesisCloseToken : Token {
    override fun asText(): CharSequence = ")"

    override fun equals(other: Any?): Boolean = other != null && this::class == other::class

    override fun hashCode(): Int = this::class.hashCode()
}
