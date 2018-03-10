
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.fullast

/**
 * Type for unary operations.
 */
interface UnaryOperationAST : BinaryAST

/**
 * Implementation for the negation unary operation (-x).
 */
data class NegateOperationAST(val target: ExpressionAST) : UnaryOperationAST {

    /**
     * {@see HasTextRepresentation.asText()}.
     */
    override fun asText(): CharSequence = "-(${target.asText()})"
}
