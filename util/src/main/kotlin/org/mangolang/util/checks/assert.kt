package org.mangolang.util.checks

/**
 * When I find a better assert in kotlin-common or a well-maintained library, I'll throw this out of delegate.
 */
fun assert(condition: Boolean, message: Lazy<String> = lazy { "Assert condition failed (without message)." }) {
    platform_assert(condition, message)
}

