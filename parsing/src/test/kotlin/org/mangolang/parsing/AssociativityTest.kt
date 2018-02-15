
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.fullast.IntegerAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.mock.FixedTokenStream
import kotlin.test.Test

/**
 * Test that 1 + 2 + 3 is parsed as 1 + (2 + 3).
 */
class AssociativityTest {
    @Test
    fun testAssociativity() {
        assertParse(
                ConcreteBinaryOperation(
                        IntegerAST(IntegerToken(2)),
                        ConcreteBinaryOperator(OperatorToken("+")),
                        ConcreteBinaryOperation(
                                IntegerAST(IntegerToken(3)),
                                ConcreteBinaryOperator(OperatorToken("+")),
                                ConcreteBinaryOperation(
                                        IntegerAST(IntegerToken(4)),
                                        ConcreteBinaryOperator(OperatorToken("+")),
                                        IntegerAST(IntegerToken(5))
                                )
                        )
                ),
                FixedTokenStream(listOf(
                        IntegerToken(2),
                        OperatorToken("+"),
                        IntegerToken(3),
                        OperatorToken("+"),
                        IntegerToken(4),
                        OperatorToken("+"),
                        IntegerToken(5)
                ))
        )
    }
}
