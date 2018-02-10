package org.mangolang.parsing

import org.mangolang.fullast.ConcreteBinaryOperation
import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.ConcreteBinaryOperator
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.token.mock.FixedTokenStream
import org.mangolang.util.errors.mock.MockListener
import kotlin.test.Test

class ParseAdditionTest {
    @Test
    fun testParseAddition() {
        var li: TokenStream
        li = FixedTokenStream(listOf(
                IntegerToken(7),
                OperatorToken("+"),
                IntegerToken(7)
        ))
        assertParse(
                ConcreteBinaryOperation(IntegerAST(IntegerToken(7)),
                        ConcreteBinaryOperator(OperatorToken("+")), IntegerAST(IntegerToken(7))),
                parse(MockListener(), li)
        )
        li = FixedTokenStream(listOf(
                IntegerToken(7),
                OperatorToken("-"),
                IntegerToken(7)
        ))
        assertParse(
                ConcreteBinaryOperation(IntegerAST(IntegerToken(7)),
                        ConcreteBinaryOperator(OperatorToken("-")), IntegerAST(IntegerToken(7))),
                parse(MockListener(), li)
        )
    }
}

