
import org.mangolang.common.Name
import org.mangolang.common.InvalidNameException
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

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
            assertEquals(inp, Name(inp).value)
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
                Name(inp)
            }
        }
    }
}


