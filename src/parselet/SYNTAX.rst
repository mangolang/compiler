
Mango Syntax
===============================

At this stage of development, the implementation is perhaps a more up-to-date reference than this document, since the syntax has not been finalized.

But for reference, this is the grammar::

TODO: right hand sides should go back up to Expression?

    Assignment:
        | Identifier "=" Expression
        | Addition

    #TODO: arguments can also be passed as name=value, but only after positional arguments
    CommaSeparatedExpressions:
        | Expression [("," | "\n") Expression]* ","
        | Expression [("," | "\n") Expression]*
        | EMPTY

    Expression:
        Addition

    Addition:
        | Multiplication ("+" | "-") Addition
        | Multiplication

    Multiplication:
        | UnaryOperation ("*" | "/") Multiplication
        | UnaryOperation

    UnaryOperation:
        | "-" UnaryOperation
        | Call

    Call:
        | Index "(" CommaSeparatedExpressions ")"
        | Index

    Index:
        | Variable "[" Expression "]"
        | Variable

    Variable:
        | IDENTIFIER
        | Literal

    Literal:
        | INTEGER
        | REAL
        | BOOL
        | TEXT
        | Group

    Group:
        | "(" Expression ")"
        | EMPTY

