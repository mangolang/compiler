package org.mangolang.fullast

import org.mangolang.token.IntegerToken

class BinaryOperationAST(
        val left: ExpressionAST,
        val operator: BinaryOperatorAST,
        val right: ExpressionAST): ExpressionAST {

    override fun asText(): CharSequence {
        return "(${left.asText()}${operator.asText()}${right.asText()})"
    }
}

