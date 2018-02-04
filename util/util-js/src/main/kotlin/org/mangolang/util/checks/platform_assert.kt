package org.mangolang.util.checks

@Suppress("NOTHING_TO_INLINE")
actual inline internal fun platform_assert(condition: Boolean, message: Lazy<String>) {
    if (!condition) {
        // TODO: this should only happen in debug mode
        throw AssertionError(message.value)
    }
}
