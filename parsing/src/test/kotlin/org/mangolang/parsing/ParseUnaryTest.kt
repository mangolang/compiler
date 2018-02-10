package org.mangolang.parsing

import org.mangolang.fullast.IntegerAST
import org.mangolang.fullast.NegateOperationAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream
import org.mangolang.token.mock.FixedTokenStream
import org.mangolang.util.errors.mock.MockListener
import kotlin.test.Test

class ParseUnaryTest {
    @Test
    fun testParseUnary() {
        var li: TokenStream
        li = FixedTokenStream(listOf(
                OperatorToken("+"),
                IntegerToken(7)
        ))
        assertParse(
                IntegerAST(IntegerToken(7)),
                parse(MockListener(), li)
        )
        li = FixedTokenStream(listOf(
                OperatorToken("-"),
                IntegerToken(7)
        ))
        assertParse(
                NegateOperationAST(IntegerAST(IntegerToken(7))),
                parse(MockListener(), li)
        )
    }
}

