package org.mangolang.util.escape

/**
 * Escape a string for display as a double-quoted, single-line string.
 */
@Suppress("ExpressionBodySyntax")
fun toStringLiteral(txt: String): String {
    // LATER: improve performance
    return '"' + txt.replace("\\", "\\\\").replace("\n", "\\n")
            .replace("\"", "\\\"") + '"'
    // return '"' + Regex("(\"|\\n|\\)").replace(txt, "") + '"'
}
