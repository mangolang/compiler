package org.mangolang.util

/**
 * Marks types which can be represented as a source-like text (unambiguously but without metadata).
 */
interface HasTextRepresentation {
    /**
     * Return a text representation of the object, including delegates recursively.
     */
    fun asText(): CharSequence
}
