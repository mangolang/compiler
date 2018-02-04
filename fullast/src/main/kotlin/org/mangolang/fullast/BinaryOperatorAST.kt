package org.mangolang.fullast

import org.mangolang.token.OperatorToken

interface BinaryOperatorAST: ExpressionAST

class ConcreteBinaryOperator(public val token: OperatorToken): BinaryOperatorAST {
    public val symbol = token.symbol
    // TODO: perhaps convert the symbol to a type (but maybe needs to be extensible in the future)

    override fun asText(): CharSequence {
        return token.asText()
    }

    val isAddSub: Boolean get() = token.isAddSub
    val isMultDiv: Boolean get() = token.isMultDiv
}

