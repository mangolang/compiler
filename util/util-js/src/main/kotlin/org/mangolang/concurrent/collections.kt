package org.mangolang.concurrent

actual fun<K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V> {
    return mutableMapOf(*pairs)
}


