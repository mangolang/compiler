package org.mangolang.fullast

import org.mangolang.token.IntegerToken
import org.mangolang.util.HASH_CODE_MULT

/**
 * Type for literal values of various types.
 */
interface LiteralAST : UnaryOperationAST

/**
 * Integer literal.
 */
data class IntegerAST(val token: IntegerToken) : LiteralAST {
    val value = token.value

    override fun asText(): CharSequence = token.asText()

    override fun equals(other: Any?): Boolean =
            other is IntegerAST && this::class == other::class && this.value == other.value

    override fun hashCode(): Int = this::class.hashCode() + HASH_CODE_MULT * value
}
