
Mango Syntax
===============================

At this stage of development, the implementation is perhaps a more up-to-date reference than this document, since the syntax has not been finalized.

But for reference, this is the grammar::

    Expression -> Assignment

    Assignment -> Identifier "=" Expression
        | Addition

    Addition -> Multiplication ("+" | "-") Addition
        | Multiplication

    Multiplication -> UnaryOperation ("*" | "/") Multiplication
        | UnaryOperation

    UnaryOperation -> NegateOperation
        | "+" UnaryOperation
        | Literal

    NegateOperation -> "-" UnaryOperation

    Literal -> INTEGER
        | "(" Expression ")"
