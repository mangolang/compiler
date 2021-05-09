
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

So note that code is only compiled when needed. Which in turn means that a program can be valid if it contains unused parts that would be invalid if they were used (in practise, just no guarantees about such code, it may be accepted or rejected).

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

*Note: at a later point, this will also include macros.*

*Note: this is for functions; maybe records and unions work differently.*

There are multiple levels of compiled-ness, each with it's own cache invalidation:

* Unvisited file.
* File's signatures parsed, bodies lexed.

    * Only reached when a file is imported and referenced
    * Triggered when imported, only referenced unit(s) compiled further.
    * Timestamp can be cache key. Is anything cached beyond source?

* Body parsed, references to dependencies known.

    * Dependencies are known but may not be at this stage yet.
    * Level is necessary: previous level does not know dependencies, next level must know dependency types. Something must be inbetween while awaiting dependency compile.
    * Perhaps no caching at this level.

* IR: body and dependencies type-checked.

    * At this point, bi-directional dependencies are known. Which is important for cache invalidation.
    * Dependencies must be at this same level before it can start.
    * Types checked, including lifetimes.
    * Cache key is file's + signature.

* Inlined / optimized.

    * Some optimizations like inlining can change or remove binary interface, so separate stage.
    * Cache: difficult - could be worth caching because slow, but optimizations depend on other modules (i.e. nr of call sites).
    * Still in same IR format.

* Compiled to target X.

    * Not sure if this is per function etc. Perhaps it's per package, or everthing together.
    * Note there could be several of these if several targets, so add some param.
    * Perhaps not a separate step for optimizing, no purpose to separate.

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
    * Must be a mechanism to wake up a task after all its dependencies are done. Either callbacks or just careful push order.
    * Tasks must be idempotent, so when a task is needed it can just be added without checking if it exists.
    * Implemented as concurrent queue, not persisted

Some thoughts:

* Cache invalidation relies on bi-directional dependencies. Different levels of invalidation, corresponding to levels: signature, binary api, inlined.
* Number of references for inlining etc should be counted per project, but the cache is shared.
* How would this work with profile-guided optimization?
* Cache format is not stable between compiler versions; new compiler = clean compile. But metadata should probably remain readable, to know the version etc.
* How and when is disp space reclaimed?


