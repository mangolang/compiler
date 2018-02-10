package org.mangolang.parsing

import org.mangolang.fullast.FullAST

/**
 * Check that a [TokenStream] is parsed successfully.
 */
fun assertParse(actual: FullAST, expected: FullAST) {
    if (actual != expected) {
        throw AssertionError("Parse result did not match expectation!\n\n" +
                "actual:   ${actual.asText()}\nexpected: ${expected.asText()}\n\n" +
                "actual:   ${actual}\nexpected: ${expected}"
        )
    }
}

