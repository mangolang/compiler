
Mango Syntax
===============================

At this stage of development, the implementation is perhaps a more up-to-date reference than this document, since the syntax has not been finalized.

But for reference, this is the grammar::

    NOTE: Is this correct syntax?
    CommaSeparatedExpressions ->
        | Expression [("," | "\n") Expression]* ","
        | Expression [("," | "\n") Expression]*
        | EMPTY

    Expression -> Assignment

    Assignment -> Identifier "=" Expression
        | Addition

    Addition -> Multiplication ("+" | "-") Addition
        | Multiplication

    Multiplication -> UnaryOperation ("*" | "/") Multiplication
        | UnaryOperation

    UnaryOperation -> "-" UnaryOperation
        | FunctionCall

    FunctionCall -> IndexedIdentifier "(" CommaSeparatedExpressions ")"
        | IndexedIdentifier

    IndexedIdentifier -> PureIdentifier "[" Expressions "]"
        | PureIdentifier

    PureIdentifier -> IDENTIFIER
        | Literal

    Literal -> INTEGER
        | REAL
        | BOOL
        | TEXT
        | Group

    Group -> "(" Expression ")"
        | EMPTY

