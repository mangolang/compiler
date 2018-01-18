package org.mangolang.common

class Name(value: String) {
    val value: String /* Interned if platform-supported. */

    init {
        if (value in setOf("function", "return", "if", "then", "else", "endif")) {
            throw InvalidNameException("Reserved name: {}" + value)
        }
        if (Regex("^[a-zA-Z_][a-zA-Z0-9_]*$").containsMatchIn(value)) {
            this.value = internStringIfPossible(value)
        } else {
            throw InvalidNameException("Not a valid name: ${value}")
        }
    }

    override fun toString(): String {
        return value
    }

    override fun hashCode(): Int {
        return value.hashCode() * 3
    }

    override fun equals(other: Any?): Boolean {
        if (other !is Name) {
            return false
        }
        return comparePossiblyInlinedStrings(value, other.value)
    }
}

class InvalidNameException(txt: String): Exception(txt) {}


