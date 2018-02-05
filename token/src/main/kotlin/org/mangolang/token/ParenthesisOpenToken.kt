package org.mangolang.token

// TODO: add `data`?
class ParenthesisOpenToken: Token {
    override fun asText(): CharSequence {
        return "("
    }
}


