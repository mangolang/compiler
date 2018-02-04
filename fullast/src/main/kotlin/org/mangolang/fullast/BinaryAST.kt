package org.mangolang.fullast

//import org.mangolang.util.checks.assert
import org.mangolang.util.checks.assert

interface BinaryAST: ExpressionAST

class ConcreteBinaryOperation(
        val left: BinaryAST,
        val operator: ConcreteBinaryOperator,
        val right: BinaryAST): BinaryAST {

    init {
        // TODO: maybe change to 'assert' but not available on common atm
        assert(operator.isAddSub || operator.isMultDiv,
                lazy { "Expected operator +, -, * or / for ConcreteBinaryOperation" })
    }

    override fun asText(): CharSequence {
        return "(${left.asText()}${operator.asText()}${right.asText()})"
    }
}

//class ConcreteMultiplicationOperation(
//        val left: MultiplicationAST,
//        val operator: ConcreteBinaryOperator,
//        val right: MultiplicationAST): AdditionAST {
//
//    init {
//        // TODO: maybe change to 'assert' but not available on common atm
//        assert(operator.symbol == "*" && operator.symbol == "/",
//                lazy { "Expected operator * or / for ConcreteAdConcreteMultiplicationOperationditionOperation" })
//    }
//
//    override fun asText(): CharSequence {
//        return "(${left.asText()}${operator.asText()}${right.asText()})"
//    }
//}

