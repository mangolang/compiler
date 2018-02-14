
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * Token for an integer literal.
 */
data class IntegerToken(val value: Int) : Token {
    override fun asText(): CharSequence = value.toString()
}
