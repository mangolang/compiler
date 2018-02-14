
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.concurrent

/**
 * Javascript implementation of [concurrentMapOf].
 */
@Suppress("ExpressionBodySyntax")
actual fun <K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V> {
    return mutableMapOf(*pairs)
}
