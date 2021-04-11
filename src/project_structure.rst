
Project structure
===============================

Compilation broadly proceeds in these steps, with representations in **bold** and operations in *italics*:

1. **Text** in source files or memory
2. *lexing*: recognize words from the language
3. **Lexemes**
4. *parsing*: connect the lexemes
5. **Parselet** (full abstract syntax tree)
6. *semanticate* (semantic analysis): check types and signatures, connect variables
7. **Sem** variables are connected but code not validated
8. *checking* Do type checking and other static analysis
9. **IR**
10. *mango.optimizing* platform-independent optimizations
11. **IR** (still)
12. *generation*
13. **target** representation at low or high level, depending on target; also optimized and preliminary mode
14. *optimizing* platform-dependent optimizations
15. **target** (still)
16. *writing*

The steps after IR happen in specific subprojects depending on the backend.
