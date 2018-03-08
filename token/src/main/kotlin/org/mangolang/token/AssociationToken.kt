
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * Equals symbol, which is used for associating a value with an identifier,
 * as well as in-place operations like *=, += etc.
 */
data class AssociationToken(val symbol: String = "=") : Token {
    override fun asText(): CharSequence = " = "
}
