
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.concurrent

import java.util.concurrent.ConcurrentHashMap

/**
 * JVM implementation of [concurrentMapOf].
 */
actual fun <K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V> {
    val map = ConcurrentHashMap<K, V>()
    for (pair in pairs) {
        map[pair.first] = pair.second
    }
    return map
}
