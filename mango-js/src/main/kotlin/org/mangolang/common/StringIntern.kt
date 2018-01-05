package org.mangolang.common

/**
 * Although string interning is not explicitly implemented for javascript,
 * some javascript implementations will still perform it automatically.
 */

actual public inline fun internStringIfPossible(str: String): String {
    return str
}

actual public inline fun comparePossiblyInlinedStrings(one: String, two: String): Boolean {
    return one == two
}


