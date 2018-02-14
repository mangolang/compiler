
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.errors

/**
 * This [ProblemListener] throws an [TooManyProblems] error when a certain number of errors or warnings have been registered.
 */
class ThresholdedListener(val maxErrors: Int? = 1, val maxWarnings: Int? = null) : ProblemListener {

    private val errors: MutableList<CompileError> = mutableListOf()
    private val warnings: MutableList<CompileWarning> = mutableListOf()

    override fun error(err: CompileError) {
        errors.add(err)
        if (maxErrors != null && errors.size >= maxErrors) {
            throw TooManyProblems("Stopping because the limit of $maxErrors errors was reached (${warnings.size} warnings).")
        }
    }

    override fun warning(warn: CompileWarning) {
        warnings.add(warn)
        if (maxWarnings != null && warnings.size >= maxWarnings) {
            throw TooManyProblems("Stopping because the limit of $maxWarnings warnings was reached (${errors.size} errors).")
        }
    }

    /**
     * This exception is thrown to terminate the compilation when there have been too many problems.
     */
    class TooManyProblems(msg: String) : Exception(msg)
}
