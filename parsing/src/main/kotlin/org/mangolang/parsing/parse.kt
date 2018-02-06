package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.token.TokenStream
import org.mangolang.util.checks.require

/**
 * This is the entry point for the recursive descent parser. It will
 * try to match a pattern in the token stream, producing nested
 * abstract syntax tree nodes.
 */
public fun parse(tokens: TokenStream): ExpressionAST {
    val expression = parseExpression(tokens)
    require(tokens.peek() == null, lazy { "Stopped before end of input" })
    return expression
}

