package org.mangolang.fullast

import org.mangolang.token.OperatorToken

/**
 * Type for binary operators (not operations; just the operator itself).
 */
interface BinaryOperatorAST: ExpressionAST

/**
 * A binary operator.
 */
data class ConcreteBinaryOperator(public val token: OperatorToken): BinaryOperatorAST {
    val symbol = token.symbol

    override fun asText(): CharSequence = token.asText()

    val isAddSub: Boolean get() = token.isAddSub
    val isMultDiv: Boolean get() = token.isMultDiv
}

