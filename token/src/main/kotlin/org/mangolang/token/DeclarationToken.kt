
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * Keyword used to indicate a declaration.
 */
data class DeclarationToken(val isMutable: Boolean = false) : Token {
    override fun asText(): CharSequence = if (isMutable) "let " else "let mut "
}
