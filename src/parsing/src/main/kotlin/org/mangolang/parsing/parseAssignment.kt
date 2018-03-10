
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.parsing

import org.mangolang.fullast.ExpressionAST
import org.mangolang.fullast.Assignment
import org.mangolang.fullast.UnparseableAST
import org.mangolang.fullast.Variable
import org.mangolang.token.AssociationToken
import org.mangolang.token.IdentifierToken
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.ProblemListener
import org.mangolang.util.text.Message

/**
 * Parse an expression being bound to an identifier or pattern.
 *
 * Much like an assignment, but including e.g. arguments.
 */
fun parseBinding(listener: ProblemListener, tokens: TokenStream): ExpressionAST {
    if (tokens.peek() !is IdentifierToken) {
        return parseAddition(listener, tokens)
    }
    val assignee = Variable(tokens.take() as IdentifierToken)
    val maybeAssignment = tokens.take()
    if (maybeAssignment !is AssociationToken) {
        listener.error(SyntaxError(Message("Found token $maybeAssignment after " +
                "$assignee, but expected an assignment."), maybeAssignment))
        return UnparseableAST(maybeAssignment)
    }
    val value = parseExpression(listener, tokens)
    return Assignment(assignee, value)
}
