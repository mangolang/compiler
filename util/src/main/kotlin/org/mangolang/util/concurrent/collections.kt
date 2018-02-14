package org.mangolang.util.concurrent

/**
 * If the platform needs and has it, returns a concurrent mutable map with the given values.
 */
expect fun <K, V> concurrentMapOf(vararg pairs: Pair<K, V>): MutableMap<K, V>
