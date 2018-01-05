package org.mangolang.common

actual public inline fun internStringIfPossible(str: String): String {
    return str.intern()
}

actual public inline fun comparePossiblyInlinedStrings(one: String, two: String): Boolean {
    return one === two
}


