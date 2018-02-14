
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.fullast

import org.mangolang.token.Token

/**
 * A part of the syntax tree that could not be parsed.
 */
class UnparseableAST(val token: Token? = null) : ExpressionAST {
    override fun asText(): CharSequence = "???unparseable???"

    /**
     * Unparseable content is not equal to other unparseable content (like NaN).
     */
    override fun equals(other: Any?): Boolean = this === other

    override fun hashCode(): Int = this::class.hashCode()
}
