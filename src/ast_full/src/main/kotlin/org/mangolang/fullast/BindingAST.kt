
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.fullast

/**
 * Type for an association, e.g. assignment.
 */
interface BindingAST : ExpressionAST

/**
 * Implement a operation on two targets.
 */
data class Assignment(
        val assignee: ExpressionAST,
        val value: ExpressionAST) : BindingAST {

    /**
     * {@see HasTextRepresentation.asText()}.
     */
    override fun asText(): CharSequence =
            "(${assignee.asText()} = (${value.asText()}))"
}
