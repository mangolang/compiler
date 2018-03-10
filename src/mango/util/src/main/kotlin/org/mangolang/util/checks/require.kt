
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.checks

/**
 * When I find a better require in kotlin-common or a well-maintained library, I'll throw this out of delegate.
 */
fun require(condition: Boolean, message: Lazy<String> = lazy { "Assert condition failed (without message)." }) {
    if (!condition) {
        throw AssertionError(message.value)
    }
}
