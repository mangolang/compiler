package org.mangolang.util.errors

/**
 * Represents a compilation problem, either error or warning level.
 */
interface CompileProblem {
    /**
     * Show the basic textual representation of the problem.
     */
    fun brief(): CharSequence

    /**
     * Show a detailed textual representation of the problem, including context.
     */
    fun detailed(): CharSequence
}

/**
 * Represents a serious compilation problem that prevents results from being as requested.
 */
interface CompileError: CompileProblem

/**
 * Represents a less serious compilation problem that can produce results, but possibly not as intended.
 */
interface CompileWarning: CompileProblem


