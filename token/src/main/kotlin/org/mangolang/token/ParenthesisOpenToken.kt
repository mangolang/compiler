package org.mangolang.token

/**
 * Closing parenthesis: ``(``.
 */
class ParenthesisOpenToken : Token {
    override fun asText(): CharSequence = "("

    override fun equals(other: Any?): Boolean = other != null && this::class == other::class

    override fun hashCode(): Int = this::class.hashCode()
}
