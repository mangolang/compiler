
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.errors

import kotlin.test.Test
import kotlin.test.assertFailsWith

/**
 * Mock class that implement [CompileError].
 */
internal class MockCompileError : CompileError {
    override fun detailed(): CharSequence = "detailed"
    override fun brief(): CharSequence = "brief"
}

/**
 * Mock class that implement [CompileWarning].
 */
internal class MockCompileWarning : CompileWarning {
    override fun detailed(): CharSequence = "detailed"
    override fun brief(): CharSequence = "brief"
}

/**
 * Test the [ThresholdListener]
 */
class ThresholdListenerTest {

    // LATER: add tests for loggers when those are added

    @Test
    fun testMaximumErrors() {
        val listener = ThresholdedListener(2, 2)
        listener.error(MockCompileError())
        assertFailsWith(ThresholdedListener.TooManyProblems::class) {
            listener.error(MockCompileError())
        }
    }

    @Test
    fun testMaximumWarnings() {
        val listener = ThresholdedListener(2, 2)
        listener.warning(MockCompileWarning())
        assertFailsWith(ThresholdedListener.TooManyProblems::class) {
            listener.warning(MockCompileWarning())
        }
    }
}
