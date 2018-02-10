package org.mangolang.token

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

import org.mangolang.token.mock.FixedTokenStream


class TokenEqualityTest {
    @Test
    fun testTokenEquality() {
        val twinOne = FixedTokenStream(listOf(
                ParenthesisOpenToken(),
                IntegerToken(7),
                OperatorToken("*"),
                RealToken(7.3),
                ParenthesisCloseToken(),
                StringToken("ok")
        ))
        val twinTwo = FixedTokenStream(listOf(
                ParenthesisOpenToken(),
                IntegerToken(7),
                OperatorToken("*"),
                RealToken(7.3),
                ParenthesisCloseToken(),
                StringToken("ok")
        ))
        assertEquals(twinOne, twinTwo)
        assertEquals(twinOne.hashCode(), twinTwo.hashCode())
        twinOne.take()
        assertEquals(twinOne, twinTwo)
        assertEquals(twinOne.hashCode(), twinTwo.hashCode())
    }

    @Test
    fun testTokenInequality() {
        assertNotEquals<Token>(ParenthesisOpenToken(), ParenthesisCloseToken())
        assertNotEquals(IntegerToken(7), IntegerToken(8))
        assertNotEquals(RealToken(7.00001), RealToken(7.00002))
        assertNotEquals(StringToken("hello"), StringToken("hallo"))
        assertNotEquals(OperatorToken("*"), OperatorToken("/"))
    }
}

