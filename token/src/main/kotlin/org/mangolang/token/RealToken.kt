
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * Token for a double literal (including exponential notation).
 */
data class RealToken(public val value: Double) : Token {

    override fun asText(): CharSequence = value.toString()
}
