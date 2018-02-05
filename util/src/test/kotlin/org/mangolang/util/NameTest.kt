
import org.mangolang.util.InvalidNameException
import org.mangolang.util.Name
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class NameTest {
    @Test
    fun testValidNames() {
        val valid = listOf(
                "a",
                "z",
                "A",
                "Z",
                "a0",
                "a1234567890",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "_",
                "hello_world",
                "_0"  /* '_0' is a string, '0_' is an int. */
        )
        for (inp in valid) {
            /* Mostly this checks that there is no exception when creating Name. */
            assertEquals(inp, Name.new(inp).name)
        }
    }

    @Test
    fun testInvalidNames() {
        val invalid = listOf(
                "0",
                "9",
                "01234567890123456789",
                "0_",  /* '_0' is a string, '0_' is an int. */
                "0a",
                "0ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "hello world",
                "hello-world",
                " ",
                "\t",
                "\n",
                "~",
                "!",
                "@",
                "#",
                "$",
                "€",
                "%",
                "^",
                "&",
                "*",
                "(",
                ")",
                "-",
                "+",
                "=",
                "}",
                "}",
                "[",
                "]",
                ":",
                ";",
                "\"",
                "'",
                "\\",
                "|",
                "/",
                "<",
                ">",
                ",",
                ".",
                "/",
                "?",
                "你好"  /* Might be allowed in the future, but not yet. */
        )
        for (inp in invalid) {
            /* Mostly this checks that there is no exception when creating Name. */
            assertFailsWith(InvalidNameException::class, "'${inp}' should be invalid") {
                Name.new(inp)
            }
        }
    }

    @Test
    fun testNameFlyweight() {
        /* Use triple-equals to check identity. */
        assertTrue(Name.new("Hello") === Name.new("Hello"))
        assertFalse(Name.new("Hello") === Name.new("Goodbye"))
    }
}


