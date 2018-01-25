package org.mangolang.fullast

import org.mangolang.token.IntegerToken

class IntegerAST(public val token: IntegerToken): LiteralAST {
    public val value = token.value

    override fun asText(): CharSequence {
        return token.asText()
    }
}

