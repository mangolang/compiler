package org.mangolang.fullast

import org.mangolang.token.OperatorToken

interface BinaryOperatorAST: ExpressionAST

data class ConcreteBinaryOperator(public val token: OperatorToken): BinaryOperatorAST {
    public val symbol = token.symbol

    override fun asText(): CharSequence {
        return token.asText()
    }

    val isAddSub: Boolean get() = token.isAddSub
    val isMultDiv: Boolean get() = token.isMultDiv
}

