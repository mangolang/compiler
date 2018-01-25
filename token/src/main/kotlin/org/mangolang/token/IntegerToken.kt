package org.mangolang.token

class IntegerToken(public val value: Int): Token {
    override fun asText(): CharSequence {
        return value.toString()
    }
}


