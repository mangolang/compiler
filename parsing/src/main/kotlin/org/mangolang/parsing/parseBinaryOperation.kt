package org.mangolang.parsing

import org.mangolang.fullast.BinaryOperationAST
import org.mangolang.fullast.BinaryOperatorAST
import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.IntegerAST
import org.mangolang.token.IntegerToken
import org.mangolang.token.OperatorToken
import org.mangolang.token.TokenStream

fun parseBinaryOperation(tokens: TokenStream): ExpressionAST {
    val first = tokens.peek()
    if (first is IntegerToken) {  // TODO this should probably try to parse an integer or value or something, instead of checking inline
        tokens.take()
        val next = tokens.peek()
        if (next is OperatorToken && next.symbol in setOf("+", "-", "*", "/", "%")) {  // TODO this should probably try to parse a binary operator
            tokens.take()
            return BinaryOperationAST(
                    IntegerAST(first),
                    BinaryOperatorAST(next),
                    parseGroup(tokens)
            )
        } else {
            return IntegerAST(first)
        }
    }
    TODO("Expected an expression, but found ${first}, ${tokens.peek()}")
}



