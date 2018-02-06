package org.mangolang.util

import org.mangolang.util.concurrent.concurrentMapOf

/**
 * This class represents the name of an identifier (variable, function, struct...).
 *
 * - Create instances using `new` instead of through the constructor, to use flyweight pattern.
 * - It's up to the parser to disallow keywords as names.
 */
public final class Name private constructor(val name: String) {
    companion object {
        private val interned: MutableMap<String, Name> = concurrentMapOf()

        /**
         * Attempts to create a new Name object.
         *
         * If the name is not valid, [InvalidNameException] is throws.
         *
         * If an object already exists for the name, the existing object is returned.
         */
        public fun new(text: String): Name {
            if (text !in interned) {
                val res = validate(text)
                if (res is Failure<Unit, String>) {
                    throw InvalidNameException(res.error)
                }
                interned[text] = Name(text)
            }
            return interned[text]!!
        }

        private val validIdentifier = Regex("^[a-zA-Z_][a-zA-Z0-9_]*$")
        private val startsWithDigit = Regex("^[0-9]")

        /**
         * Check whether the input value is a valid identifier. Returns an explanation if not.
         */
        public fun validate(value: String): Result<Unit, String> {
            if (validIdentifier.containsMatchIn(value)) {
                return Success<Unit, String>(Unit)
            }
            if (startsWithDigit.containsMatchIn(value)) {
                return Failure<Unit, String>("Name '${value}' is invalid. " +
                        "Names should not start with a digit.")
            } else {
                return Failure<Unit, String>("Name '${value}' is invalid. " +
                        "Names should consist of letters, numbers and underscores.")
            }
        }
    }

    override fun toString(): String {
        return name
    }

    final override fun hashCode(): Int {
        return name.hashCode() * 3
    }

    final override fun equals(other: Any?): Boolean {
        /* Due to flyweight pattern, this is just an identity comparison. */
        return this === other
    }
}

class InvalidNameException(txt: String): Exception(txt) {}


