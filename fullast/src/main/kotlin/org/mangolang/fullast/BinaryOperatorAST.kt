package org.mangolang.fullast

import org.mangolang.token.OperatorToken
import org.mangolang.util.HASH_CODE_MULT

/**
 * Type for binary operators (not operations; just the operator itself).
 */
interface BinaryOperatorAST : ExpressionAST

/**
 * A binary operator.
 */
data class ConcreteBinaryOperator(val token: OperatorToken) : BinaryOperatorAST {
    val symbol = token.symbol

    override fun asText(): CharSequence = token.asText()

    val isAddSub: Boolean get() = token.isAddSub
    val isMultDiv: Boolean get() = token.isMultDiv

    override fun equals(other: Any?): Boolean =
            other is ConcreteBinaryOperator && this::class == other::class && this.symbol == other.symbol

    override fun hashCode(): Int = this::class.hashCode() + HASH_CODE_MULT * token.hashCode()
}
