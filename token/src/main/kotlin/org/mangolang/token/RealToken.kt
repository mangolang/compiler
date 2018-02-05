package org.mangolang.token

data class RealToken(public val value: Double): Token {
    override fun asText(): CharSequence {
        return value.toString()
    }
}


