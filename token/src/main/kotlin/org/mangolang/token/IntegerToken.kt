package org.mangolang.token

data class IntegerToken(public val value: Int): Token {
    override fun asText(): CharSequence {
        return value.toString()
    }
}


