package org.mangolang.fullast

import org.mangolang.util.HasTextRepresentation

/**
 * An element in the full abstract syntax tree of the language.
 * Produced by the parser and converted by the reducer to [CoreAST].
 * In case the [CoreAST] and [FullAST] elements are the same, place it in [core].
 */
interface FullAST: HasTextRepresentation

