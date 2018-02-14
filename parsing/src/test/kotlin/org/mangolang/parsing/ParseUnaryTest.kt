package org.mangolang.parsing

import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.NegateOperationAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.mock.FixedTokenStream
import kotlin.test.Test

class ParseUnaryTest {
    @Test
    fun testParseUnary() {
        assertParse(
                IntegerAST(IntegerToken(7)),
                FixedTokenStream(listOf(
                        OperatorToken("+"),
                        IntegerToken(7)
                ))
        )
        assertParse(
                NegateOperationAST(IntegerAST(IntegerToken(7))),
                FixedTokenStream(listOf(
                        OperatorToken("-"),
                        IntegerToken(7)
                ))
        )
    }
}
