package org.mangolang.token

data class StringToken(public val value: String): Token {
    override fun asText(): CharSequence {
        return "\"${value}\""
        // TODO: do escaping
    }
}


