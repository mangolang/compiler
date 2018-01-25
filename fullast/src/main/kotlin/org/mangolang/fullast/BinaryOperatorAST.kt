package org.mangolang.fullast

import org.mangolang.token.OperatorToken

class BinaryOperatorAST(public val token: OperatorToken): FullAST {
    public val symbol = token.symbol
    // TODO: perhaps convert the symbol to a type
    // (but maybe needs to be extensible in the future)

    override fun asText(): CharSequence {
        return token.asText()
    }
}

