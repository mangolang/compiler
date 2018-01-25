package org.mangolang.token

class StringToken(public val value: String): Token {
    override fun asText(): CharSequence {
        return "\"${value}\""
        // TODO: do escaping
    }
}


