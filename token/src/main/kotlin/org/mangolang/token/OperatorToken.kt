package org.mangolang.token

// This might be extended to allow adding custom tokens (or another class added)

/**
 * A token representing an operator (like ``+``, ``-``, ``*``, ``/`` etc).
 */
data class OperatorToken(public val symbol: String) : Token {

    val isNegate: Boolean get() = symbol == "-"

    val isUnaryNoop: Boolean get() = symbol == "+"

    val isAddSub: Boolean get() = symbol in setOf("+", "-")

    val isMultDiv: Boolean get() = symbol in setOf("*", "/")

    override fun asText(): CharSequence = " $symbol "
}
