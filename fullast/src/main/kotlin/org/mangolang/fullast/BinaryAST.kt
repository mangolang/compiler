package org.mangolang.fullast

import org.mangolang.util.checks.require

interface BinaryAST: ExpressionAST

data class ConcreteBinaryOperation(
        val left: ExpressionAST,
        val operator: ConcreteBinaryOperator,
        val right: ExpressionAST): BinaryAST {

    init {
        // TODO: maybe change to 'require' but not available on common atm
        require(operator.isAddSub || operator.isMultDiv,
                lazy { "Expected operator +, -, * or / for ConcreteBinaryOperation" })
    }

    override fun asText(): CharSequence {
        return "(${left.asText()}${operator.asText()}${right.asText()})"
    }
}

