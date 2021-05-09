
Project structure
===============================

Steps
-------------------------------

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

Data structure
-------------------------------

Intermediate results are kept in memory and on disk, for several reasons:

* Fast incremental compiles
* Build in steps so as to only compile what is used
* Share cache between projects (on one machine, or in CI)

Disk
...............................

The disk structure for dependencies is three-tiered:

* `dependency` / `version` / `environment`

where `environment` is e.g. debug/release, compiler version, feature flags, etc.

Inside environment, different pieces of code have different levels of compiled-ness. Things are only compiled as far as necessary for the builds they have been a part of.

The dependencies are stored in a user-wide directly (so shared between projects for the user, but not between users).

For the project itself, there is only `target`, which contains a bunch of `environment`s.

Levels
...............................

There are multiple levels of compiled-ness:

* Unvisited file.
* File's signatures parsed, bodies lexed.
* References in body resolved, type checked.
* Inlined / optimized.
* Compiled to target X.

Structure
...............................

These are the main data locations:

* The current state is in a big map

    * The key identifies a version of a file (path, but also compiler flags etc).
    * The value represents the code units in the file in various stages of compiled-ness.
    * The whole value is RwLock-ed. Care must be taken with cyclic dependencies.
    * The map can live partly in memory (concurrent map) and partly on disk (when to big, or on shutdown).

* A queue of all the tasks

    * Values are tasks to increment a code unit's level by one (i.e. parse, type-check...)
    * Implemented as concurrent queue, not persisted


