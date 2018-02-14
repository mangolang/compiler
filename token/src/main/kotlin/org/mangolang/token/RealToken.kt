package org.mangolang.token

/**
 * Token for a double literal (including exponential notation).
 */
data class RealToken(public val value: Double) : Token {

    override fun asText(): CharSequence = value.toString()
}
