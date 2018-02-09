package org.mangolang.token

/**
 * Closing parenthesis: ``)``.
 */
class ParenthesisCloseToken: Token {
    override fun asText(): CharSequence = ")"
}

