package org.mangolang.util.escape

/**
 * Escape a string for display as a double-quoted, single-line string.
 */
@Suppress("ExpressionBodySyntax")
fun toStringLiteral(txt: String): String {
    // LATER: improve performance
    // LATER: unit test and fix (incl e.g. \\n as input)
    return '"' + txt.replace("\\", "\\\\").replace("\n", "\\n")
            .replace("\"", "\\\"") + '"'
    // return '"' + Regex("(\"|\\n|\\)").replace(txt, "") + '"'
}

