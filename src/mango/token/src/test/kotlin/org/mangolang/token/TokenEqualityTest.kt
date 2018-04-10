
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

import org.mangolang.token.mock.FixedTokenStream

class TokenEqualityTest {
    @Test
    fun testTokenEquality() {
        val twin_one = FixedTokenStream(listOf(
                ParenthesisOpenToken(),
                IntegerToken(7),
                OperatorToken("*"),
                RealToken(7.3),
                ParenthesisCloseToken(),
                StringToken("ok")
        ))
        val twin_two = FixedTokenStream(listOf(
                ParenthesisOpenToken(),
                IntegerToken(7),
                OperatorToken("*"),
                RealToken(7.3),
                ParenthesisCloseToken(),
                StringToken("ok")
        ))
        assertEquals(twin_one, twin_two)
        assertEquals(twin_one.hashCode(), twin_two.hashCode())
        twin_one.take()
        assertEquals(twin_one, twin_two)
        assertEquals(twin_one.hashCode(), twin_two.hashCode())
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
