package org.mangolang.fullast

import org.mangolang.util.checks.require

/**
 * Type for binary operations.
 */
interface BinaryAST: ExpressionAST

/**
 * Implement a operation on two targets.
 */
data class ConcreteBinaryOperation(
        val left: ExpressionAST,
        val operator: ConcreteBinaryOperator,
        val right: ExpressionAST): BinaryAST {

    init {
        require(operator.isAddSub || operator.isMultDiv,
                lazy { "Expected operator +, -, * or / for ConcreteBinaryOperation" })
    }

    /**
     * {@see HasTextRepresentation.asText()}.
     */
    override fun asText(): CharSequence =
            "(${left.asText()}${operator.asText()}${right.asText()})"
}

