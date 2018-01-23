
Project structure
===============================

Compilation broadly proceeds in these steps, with representations in **bold** and operations in *italics*:

1. **Text** in source files or memory
2. *Lexing*: recognize words from the language
3. **Tokens**
4. *Parsing*: connect the tokens
5. **fullAST** (full abstract syntax tree)
6. *Reducing*: map synonymous expressions to the same construct
7. **coreAST** (reduced subset of abstract syntax tree)
8. *Semanticate* (semantic analysis): check types and signatures, connect variables
9. **sem** variables are connected but code not validated
10. *checking* Do type checking and other static analysis
11. **IR**
12. *optimizing* platform-independent optimizations
13. **IR** (still)
14. *Generation*
15. **target** representation at low or high level, depending on target; also optimized and preliminary mode
16. *PD-optimizing* platform-dependent optimizations
17. **target** (still)
18. *Writing*

Each of these has it's own sub-project.


