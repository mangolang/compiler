package org.mangolang.concurrent

import java.util.concurrent.ConcurrentHashMap

actual fun<K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V> {
    val map = ConcurrentHashMap<K, V>()
    for (pair in pairs) {
        map[pair.first] = pair.second
    }
    return map
}

