package org.mangolang.fullast

import org.mangolang.token.Token

/**
 * A part of the syntax tree that could not be parsed.
 */
// LATER: spellcheck
class UnparseableAST(val token: Token? = null): ExpressionAST {
    override fun asText(): CharSequence = "???unparseable???"
}

