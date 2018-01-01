
Project structure
===============================

Compilation broadly proceeds in these steps, with representations in **bold** and operations in *italics*:

1. **Text** in source files or memory
2. *Lexing*: recognize words from the language
3. **Tokens**
4. *Parsing*: connect the tokens
5. **fAST** (full abstract syntax tree)
6. *Uniquify*: map multiple statements to the same construct
7. **rAST** (reduced subset of abstract syntax tree)
8. *Semanticate* (semantic analysis): check types and signatures, connect variables
9. **IR** intermediary representation
10. *PI-opt* machine-independent optimizations
11. **IR** (still)
12. *Generation*
13. **target** representation at low or high level, depending on target; also optimized and preliminary mode
14. *PD-opt* platform-dependent optimizations
15. **target** (still)
16. *Writing*
17. **assembly** or other runnable target

Each of these has it's own sub-project.


