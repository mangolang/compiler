
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.mock.FixedTokenStream
import kotlin.test.Test

class ParseAdditionTest {
    @Test
    fun testParseAddition() {
        assertParse(
                ConcreteBinaryOperation(IntegerAST(IntegerToken(7)),
                        ConcreteBinaryOperator(OperatorToken("+")), IntegerAST(IntegerToken(7))),
                FixedTokenStream(listOf(
                        IntegerToken(7),
                        OperatorToken("+"),
                        IntegerToken(7)
                ))
        )
        assertParse(
                ConcreteBinaryOperation(IntegerAST(IntegerToken(7)),
                        ConcreteBinaryOperator(OperatorToken("-")), IntegerAST(IntegerToken(7))),
                FixedTokenStream(listOf(
                        IntegerToken(7),
                        OperatorToken("-"),
                        IntegerToken(7)
                ))
        )
    }
}
