
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.checks

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Test that the [org.mangolang.util.checks.require] function fails when needed.
 */
class RequireTest {
    @Test
    fun test_require() {
        var thrown = false
        try {
            require(false, lazy { "test_message" })
        } catch (err: AssertionError) {
            thrown = true
            assertEquals("test_message", err.message)
        }
        assertTrue(thrown, "'require' did not throw the expected assertion error")
        require(true, lazy { "test_message" })
    }
}
