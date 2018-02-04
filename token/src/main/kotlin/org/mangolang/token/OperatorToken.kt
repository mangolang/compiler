package org.mangolang.token

// This might be extended to allow adding custom tokens (or another class added)

class OperatorToken(public val symbol: String): Token {
    override fun asText(): CharSequence {
        return " $symbol "
    }

    val isNegate: Boolean get() {
        return symbol == "-"
    }

    val isUnaryNoop: Boolean get() {
        return symbol == "+"
    }

    val isAddSub: Boolean get() {
        return symbol in setOf("+", "-")
    }

    val isMultDiv: Boolean get() {
        return symbol in setOf("*", "/")
    }
}

