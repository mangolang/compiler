
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.token.Token
import org.mangolang.util.text.Message
import org.mangolang.util.errors.CompileError

//TODO

/**
 * An error that occurs during parsing.
 */
class SyntaxError(val msg: Message, val token: Token?) : CompileError {
    override fun brief(): CharSequence = "syntax error: $msg"

    override fun detailed(): CharSequence {
        // LATER: do the context information better
        var text = "syntax error: $msg"
        if (token != null) {
            text += "\nnear input ${token.asText()}"
        }
        return text
    }
}
