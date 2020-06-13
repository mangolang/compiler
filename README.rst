
.. image:: https://travis-ci.org/mangolang/compiler.svg?branch=master
    :target: https://travis-ci.org/mangolang/compiler

.. image:: https://deps.rs/repo/github/mangolang/compiler/status.svg
    :target: https://deps.rs/repo/github/mangolang/compiler

.. image:: https://readthedocs.org/projects/mangolang/badge/?version=latest
    :target: https://docs.mangocode.org/en/latest/

.. image:: https://img.shields.io/badge/License-Apache%202.0-blue.svg
    :target: https://opensource.org/licenses/Apache-2.0


Mango
===============================

A programming language to help you make large software projects reliable, for browsers and servers.

This is the compiler 'frontend', which does the parsing and type checking. It produces IR_, to be compiled_, to WebAssembly or interpreted_.

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

There are hundreds of pages of design notes, but the plan still lacks coherence, so is unpublished.

How to use
-------------------------------

The compiler is written in Rust. You can run it using:

    cargo run

This compiles to native code, with WebAssembly to be added later (#34).

Links
-------------------------------

* `Official website`_
* `Documentation`_
* `Code of conduct and contributing`_

.. _Official website: https://mangocode.org/
.. _`Documentation`: https://docs.mangocode.org/
.. _`Code of conduct and contributing`: https://github.com/mangolang/mango
.. _IR: https://github.com/mangolang/mango_ir
.. _compiled: https://github.com/mangolang/wasm
.. _interpreted: https://github.com/mangolang/interpreter
