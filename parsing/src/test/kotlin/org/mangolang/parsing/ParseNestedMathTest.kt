package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.NegateOperationAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.ParenthesisCloseToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.mock.FixedTokenStream
import kotlin.test.Test

class ParseNestedMathTest {
    @Test
    fun testMultiplyDoubleGroupedNegatedAdditions() {
        assertParse(
                ConcreteBinaryOperation(
                        NegateOperationAST(
                                ConcreteBinaryOperation(
                                        IntegerAST(IntegerToken(2)),
                                        ConcreteBinaryOperator(OperatorToken("+")),
                                        IntegerAST(IntegerToken(3))
                                )
                        ),
                        ConcreteBinaryOperator(OperatorToken("/")),
                        NegateOperationAST(
                                ConcreteBinaryOperation(
                                        IntegerAST(IntegerToken(4)),
                                        ConcreteBinaryOperator(OperatorToken("+")),
                                        IntegerAST(IntegerToken(5))
                                )
                        )
                ),
                FixedTokenStream(listOf(
                        OperatorToken("-"),
                        ParenthesisOpenToken(),
                        IntegerToken(2),
                        OperatorToken("+"),
                        IntegerToken(3),
                        ParenthesisCloseToken(),
                        OperatorToken("/"),
                        OperatorToken("-"),
                        ParenthesisOpenToken(),
                        IntegerToken(4),
                        OperatorToken("+"),
                        IntegerToken(5),
                        ParenthesisCloseToken()
                ))
        )
    }
}
