/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.*
import org.mangolang.token.*
import org.mangolang.token.mock.FixedTokenStream
import org.mangolang.util.Name
import kotlin.test.Test

class ParseAssignmentTest {
    @Test
    fun testParseAssignment() {
        assertParse(
                Assignment(
                        Variable(IdentifierToken(Name.new("x"))),
                        IntegerAST(IntegerToken(5))
                ),
                FixedTokenStream(listOf(
                        IdentifierToken(Name.new("x")),
                        AssociationToken("="),
                        IntegerToken(5)
                ))
        )
        assertParse(
                Assignment(
                        Variable(IdentifierToken(Name.new("x"))),
                        ConcreteBinaryOperation(IntegerAST(IntegerToken(2)),
                                ConcreteBinaryOperator(OperatorToken("*")),
                                Assignment(
                                        Variable(IdentifierToken(Name.new("y"))),
                                        ConcreteBinaryOperation(IntegerAST(IntegerToken(5)),
                                                ConcreteBinaryOperator(OperatorToken("+")),
                                                IntegerAST(IntegerToken(7))
                                        )
                                )
                        )
                ),
                FixedTokenStream(listOf(
                        IdentifierToken(Name.new("x")),
                        AssociationToken("="),
                        IntegerToken(2),
                        OperatorToken("*"),
                        IdentifierToken(Name.new("y")),
                        AssociationToken("="),
                        IntegerToken(5),
                        OperatorToken("+"),
                        IntegerToken(7)
                ))
        )
    }
}
