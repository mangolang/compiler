package org.mangolang.util.errors

/**
 * A type that different parts of the compiler can register problems.
 *
 * The registered data can be used for reporting, and may abort the compilation process.
 */
interface ProblemListener {
    /**
     * Register a [CompileError].
     */
    fun error(err: CompileError)

    /**
     * Registers a [CompileWarning].
     */
    fun warning(warn: CompileWarning)
}

