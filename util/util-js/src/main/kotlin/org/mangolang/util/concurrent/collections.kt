package org.mangolang.util.concurrent

/**
 * Javascript implementation of [concurrentMapOf].
 */
@Suppress("ExpressionBodySyntax")
actual fun <K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V> {
    return mutableMapOf(*pairs)
}
