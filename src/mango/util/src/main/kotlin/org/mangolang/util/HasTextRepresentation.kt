
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

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
