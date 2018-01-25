package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.TokenStream

fun parseExpression(tokens: TokenStream): ExpressionAST {
    // TODO: more to be added later
    // TODO: shouldn't expression parse group?
    return parseBinaryOperation(tokens)
}


