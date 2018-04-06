
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.fullast

import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

/**
 * Test that equals() and hashCode() work for AST nodes.
 *
 * Note that they should not depend on the [Token], just the actual data.
 */
class ASTEqualityTest {
    @Test
    fun testASTEquality() {
        val twin_one = ConcreteBinaryOperation(
                IntegerAST(IntegerToken(7)),
                ConcreteBinaryOperator(OperatorToken("*")),
                NegateOperationAST(IntegerAST(IntegerToken(3)))
        )
        val twin_two = ConcreteBinaryOperation(
                IntegerAST(IntegerToken(7)),
                ConcreteBinaryOperator(OperatorToken("*")),
                NegateOperationAST(IntegerAST(IntegerToken(3)))
        )
        assertEquals(twin_one, twin_two)
        assertEquals(twin_one.hashCode(), twin_two.hashCode())
    }

    @Test
    fun testASTInequality() {
        assertNotEquals<FullAST>(IntegerAST(IntegerToken(7)), NegateOperationAST(IntegerAST(IntegerToken(7))))
        assertNotEquals(IntegerAST(IntegerToken(7)), IntegerAST(IntegerToken(8)))
        assertNotEquals(NegateOperationAST(IntegerAST(IntegerToken(7))), NegateOperationAST(IntegerAST(IntegerToken(8))))
        assertNotEquals(ConcreteBinaryOperator(OperatorToken("*")), ConcreteBinaryOperator(OperatorToken("/")))
        assertNotEquals(
                ConcreteBinaryOperation(IntegerAST(IntegerToken(+7)), ConcreteBinaryOperator(OperatorToken("*")), IntegerAST(IntegerToken(3))),
                ConcreteBinaryOperation(IntegerAST(IntegerToken(-7)), ConcreteBinaryOperator(OperatorToken("*")), IntegerAST(IntegerToken(3)))
        )
    }

    @Test
    fun testUnpareableEquality() {
        var up: UnparseableAST
        up = UnparseableAST(null)
        assertEquals(up, up)
        up = UnparseableAST(IntegerToken(7))
        assertEquals(up, up)
        assertNotEquals(UnparseableAST(null), UnparseableAST(null))
        assertNotEquals(UnparseableAST(IntegerToken(7)), UnparseableAST(IntegerToken(7)))
    }
}
