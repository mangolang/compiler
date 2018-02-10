package org.mangolang.parsing

import org.mangolang.fullast.FullAST
import org.mangolang.token.TokenStream
import org.mangolang.util.errors.mock.MockListener

/**
 * Check that a [TokenStream] is parsed successfully.
 */
fun assertParse(expected: FullAST, input: TokenStream) {
    val actual = parse(MockListener(), input)
    if (actual != expected) {
        throw AssertionError("Parse result did not match expectation!\n\n" +
                "actual:   ${actual.asText()}\nexpected: ${expected.asText()}\n\n" +
                "actual:   ${actual}\nexpected: ${expected}"
        )
    }
}

