package org.mangolang.fullast

import org.mangolang.token.IntegerToken

/**
 * Type for literal values of various types.
 */
interface LiteralAST: UnaryOperationAST

/**
 * Integer literal.
 */
data class IntegerAST(public val token: IntegerToken): LiteralAST {
    val value = token.value

    override fun asText(): CharSequence = token.asText()
}

