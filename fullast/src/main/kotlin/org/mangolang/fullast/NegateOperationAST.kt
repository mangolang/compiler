package org.mangolang.fullast

/**
 * Type for unary operations.
 */
interface UnaryOperationAST: BinaryAST

/**
 * Implementation for the negation unary operation (-x).
 */
class NegateOperationAST(val target: ExpressionAST): UnaryOperationAST {

    /**
     * {@see HasTextRepresentation.asText()}.
     */
    override fun asText(): CharSequence = "-(${target.asText()})"
}

