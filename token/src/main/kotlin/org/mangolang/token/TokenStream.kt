package org.mangolang.token

/**
 * A token as produced by the lexer. Describes a single element of the syntax.
 *
 * Intentionally does not support looking ahead further than one token.
 */
interface TokenStream {
    /**
     * Peeks at an item from the stream, returning the item but not advancing the stream.
     */
    public fun peek(): Token?

    /**
     * Takes an item from the stream and returns it.
     */
    public fun take(): Token?
}

