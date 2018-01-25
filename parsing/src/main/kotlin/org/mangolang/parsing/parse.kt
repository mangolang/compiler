package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.FullAST
import org.mangolang.token.TokenStream

/**
 * This is the entry point for the recursive descent parser. It will
 * try to match a pattern in the token stream, producing nested
 * abstract syntax tree nodes.
 */
public fun parse(tokens: TokenStream): ExpressionAST {
    return parseGroup(tokens)
}


