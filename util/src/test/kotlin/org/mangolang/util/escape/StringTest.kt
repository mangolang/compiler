
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.escape

import kotlin.test.Test
import kotlin.test.assertEquals

class StringTest {
    @Test
    fun testToStringLiteral() {
        assertEquals("\"hello world\"", toStringLiteral("hello world"))
        assertEquals("\"hello\\nworld\"", toStringLiteral("hello\nworld"))
        assertEquals("\"hello\\\\ world\"", toStringLiteral("hello\\ world"))
        assertEquals("\"hello\\\"world\"", toStringLiteral("hello\"world"))
        assertEquals("\"\\\"\\\"\\\"\\n\\\\\"", toStringLiteral("\"\"\"\n\\"))
        assertEquals("\"\\\\n\"", toStringLiteral("\\n"))
        assertEquals("\"\\\\\\n\"", toStringLiteral("\\\n"))
    }
}
