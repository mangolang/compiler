package org.mangolang.parsing

import org.mangolang.fullast.IntegerAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.ParenthesisCloseToken
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.mock.FixedTokenStream
import kotlin.test.Test

class ParseGroupTest {
    @Test
    fun testParseUnary() {
        assertParse(
                IntegerAST(IntegerToken(7)),
                FixedTokenStream(listOf(
                        ParenthesisOpenToken(),
                        ParenthesisOpenToken(),
                        OperatorToken("+"),
                        ParenthesisOpenToken(),
                        ParenthesisOpenToken(),
                        IntegerToken(7),
                        ParenthesisCloseToken(),
                        ParenthesisCloseToken(),
                        ParenthesisCloseToken(),
                        ParenthesisCloseToken()
                ))
        )
    }
}

