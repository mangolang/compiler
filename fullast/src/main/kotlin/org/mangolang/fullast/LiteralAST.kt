package org.mangolang.fullast

import org.mangolang.token.IntegerToken

interface LiteralAST: UnaryOperationAST

data class IntegerAST(public val token: IntegerToken): LiteralAST {
    public val value = token.value

    override fun asText(): CharSequence {
        return token.asText()
    }
}
