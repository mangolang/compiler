
Mango Syntax
===============================

At this stage of development, the implementation is perhaps a more up-to-date reference than this document, since the syntax has not been finalized.

But for reference, this is the grammar::

    Assignment:
        | Identifier "=" Expression
        | Addition

    CommaSeparatedExpressions:
        | Expression [("," | "\n") Expression]* ","
        | Expression [("," | "\n") Expression]*
        | EMPTY

    Addition:
        | Multiplication ("+" | "-") Addition
        | Multiplication

    Multiplication:
        | UnaryOperation ("*" | "/") Multiplication
        | UnaryOperation

    UnaryOperation:
        | "-" UnaryOperation
        | FunctionCall

    IdentifierCall:
        | IndexedIdentifier "(" CommaSeparatedExpressions ")"
        | IdentifierIndex

    IdentifierIndex:
        | Value "[" Expressions "]"
        | Identifier

    Identifier:
        | Literal
        | IDENTIFIER

    Literal:
        | INTEGER
        | REAL
        | BOOL
        | TEXT
        | Group

    Group:
        | "(" Expression ")"
        | EMPTY

