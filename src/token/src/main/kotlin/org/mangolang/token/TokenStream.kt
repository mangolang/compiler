
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.token

/**
 * A mango.token as produced by the lexer. Describes a single element of the syntax.
 *
 * Intentionally does not support looking ahead further than one mango.token.
 */
interface TokenStream {
    /**
     * Peeks at an item from the stream, returning the item but not advancing the stream.
     */
    fun peek(): Token?

    /**
     * Takes an item from the stream and returns it.
     */
    fun take(): Token?
}
