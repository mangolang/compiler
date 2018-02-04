package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.ParenthesisOpenToken
import org.mangolang.token.TokenStream

fun parseExpression(tokens: TokenStream): ExpressionAST {
    return parseAddition(tokens)
//
//    val product = parseMultiplication(tokens)
//    if (product != null) {
//        return product
//    }
//    val next = tokens.take()
//    if (next is OperatorToken && next.symbol == "+") {
//        return parseMultiplication(tokens)
//    }
//    if (next is OperatorToken && next.symbol == "-") {
//        return NegateOperationAST(parseMultiplication(next))
//    }
//    TODO("expect a sum or product, but found ${next}")
}

fun parseGroupedExpression(tokens: TokenStream): ExpressionAST {
    val token = tokens.take()
    if (token !is ParenthesisOpenToken) {
        // TODO: specific error type
        throw IllegalArgumentException("Expected a grouped expression, but found ${tokens.peek()}")  // really need to stop here to avoid infinite recursion
    }
    val expression = parseExpression(tokens)
    if (tokens.peek() !is ParenthesisOpenToken) {
        // TODO: specific error type
        throw IllegalArgumentException("Did not find a closing parenthesis at end of expression; found ${tokens.peek()}")  // really need to stop here to avoid infinite recursion
    }
    tokens.take()
    return expression
}

