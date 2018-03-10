
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.fullast

import org.mangolang.token.IdentifierToken
import org.mangolang.util.HASH_CODE_MULT

/**
 * Type for literal values of various types.
 */
interface VariableAST : ExpressionAST

/**
 * Integer literal.
 */
data class Variable(val token: IdentifierToken) : VariableAST {
    val name = token.name

    override fun asText(): CharSequence = token.asText()

    override fun equals(other: Any?): Boolean =
            other is Variable && this::class == other::class && this.name == other.name

    override fun hashCode(): Int = this::class.hashCode() + HASH_CODE_MULT * name.hashCode()
}
