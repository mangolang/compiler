
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.errors.mock

import org.mangolang.util.errors.CompileError
import org.mangolang.util.errors.CompileWarning
import org.mangolang.util.errors.ProblemListener

/**
 * Mock version of [ProblemListener] which just discards all errors.
 */
class MockListener : ProblemListener {
    override fun error(err: CompileError) {}
    override fun warning(warn: CompileWarning) {}
}
