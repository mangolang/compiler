package org.mangolang.common

/**
 * If the platform supports interning, return an interned version, otherwise return the string itself.
 */
expect public inline fun internStringIfPossible(str: String): String

/**
 * Compare two possibly-interned strings (by pointer if interned, by value otherwise).
 */
expect public inline fun comparePossiblyInlinedStrings(one: String, two: String): Boolean


