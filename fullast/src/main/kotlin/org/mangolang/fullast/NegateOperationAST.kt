package org.mangolang.fullast

interface UnaryOperationAST: BinaryAST

class NegateOperationAST(val target: ExpressionAST): UnaryOperationAST {

    override fun asText(): CharSequence {
        return "-(${target.asText()})"
    }
}

