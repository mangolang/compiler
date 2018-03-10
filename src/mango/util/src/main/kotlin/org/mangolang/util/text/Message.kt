
/* Mango compiler (mangolang.org) | Apache 2.0 license, © 2018. */

package org.mangolang.util.text

/**
 * This class represents a notification message.
 */
data class Message(val value: String) {
    // LATER: add any possible restrictions
    // LATER: add automated tests

    override fun toString(): String = value
}
