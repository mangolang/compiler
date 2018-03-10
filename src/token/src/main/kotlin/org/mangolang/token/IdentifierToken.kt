
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

import org.mangolang.util.Name

/**
 * An arbitrary identifier - most any properly formatted string that isn't a keyword.
 */
data class IdentifierToken(public val name: Name) : Token {
    override fun asText(): CharSequence = "$name"
}
