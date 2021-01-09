
Project structure
===============================

Compilation broadly proceeds in these steps, with representations in **bold** and operations in *italics*:

TODO: Change 'Parselet' to 'Parselet'
TODO: Merge full and core ast, they're the same structs in different enum

Note: step 2, 3 and 4 are currently 'replaced' by a PEST parser.

1. **Text** in source files or memory
2. *Lexing*: recognize words from the language
3. **Lexemes**
4. *Parsing*: connect the lexemes
5. **Parselet** (full abstract syntax tree)
6. *Semanticate* (semantic analysis): check types and signatures, connect variables
7. **mango.sem** variables are connected but code not validated
8. *checking* Do type checking and other static analysis
9. **IR**
10. *mango.optimizing* platform-independent optimizations
11. **IR** (still)
12. *Generation*
13. **target** representation at low or high level, depending on target; also optimized and preliminary mode
14. *PD-mango.optimizing* platform-dependent optimizations
15. **target** (still)
16. *Writing*

Each of these has it's own sub-project.
