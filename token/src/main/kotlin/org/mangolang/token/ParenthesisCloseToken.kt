package org.mangolang.token

// TODO: add `data`?
class ParenthesisCloseToken: Token {
    override fun asText(): CharSequence {
        return ")"
    }
}


