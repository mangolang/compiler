
import org.mangolang.common.internStringIfPossible
import org.mangolang.common.comparePossiblyInlinedStrings
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class StringInternTest {
    @Test
    fun testEquality() {
        val a = internStringIfPossible("Hello World")
        val b = internStringIfPossible("Hello World")
        assertTrue(comparePossiblyInlinedStrings(a, b))
    }

    @Test
    fun testInequality() {
        val a = internStringIfPossible("Hello World")
        val b = internStringIfPossible("Hello Moon")
        assertFalse(comparePossiblyInlinedStrings(a, b))
    }
}


