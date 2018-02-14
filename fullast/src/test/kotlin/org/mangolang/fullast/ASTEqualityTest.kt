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
        val twinOne = ConcreteBinaryOperation(
                IntegerAST(IntegerToken(7)),
                ConcreteBinaryOperator(OperatorToken("*")),
                NegateOperationAST(IntegerAST(IntegerToken(3)))
        )
        val twinTwo = ConcreteBinaryOperation(
                IntegerAST(IntegerToken(7)),
                ConcreteBinaryOperator(OperatorToken("*")),
                NegateOperationAST(IntegerAST(IntegerToken(3)))
        )
        assertEquals(twinOne, twinTwo)
        assertEquals(twinOne.hashCode(), twinTwo.hashCode())
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
