
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.lexeme.Lexeme
import org.mangolang.util.text.Message
import org.mangolang.util.errors.CompileError

/**
 * An error that occurs during mango.parsing.
 */
class SyntaxError(val msg: Message, val lexeme: Lexeme?) : CompileError {
    override fun brief(): CharSequence = "syntax error: $msg"

    override fun detailed(): CharSequence {
        // LATER: do the context information better
        var text = "syntax error: $msg"
        if (lexeme != null) {
            text += "\nnear input ${lexeme.asText()}"
        }
        return text
    }
}
