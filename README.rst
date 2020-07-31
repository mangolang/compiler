
.. image:: https://github.com/mangolang/compiler/workflows/Check%20Mango%20compiler%20frontend/badge.svg?branch=dev
    :target: https://github.com/mangolang/compiler/actions

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

https://mangocode.org/

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

There are hundreds of pages of design notes, but the plan still lacks coherence, so is unpublished.

How to use
-------------------------------

To interact with Mango from the command line, use the CLI_ crate.

This crate is a Rust library. You cannot execute it directly, use the CLI_. Use this library if you're making another Rust tool that relies on the Mango compiler.

You can test and compile the library with Docker:

    docker build .

Or run the cargo commands from the Dockerfile yourself.

Links
-------------------------------

* `Official website`_
* `Documentation`_
* `Code of conduct and contributing`_

.. _Official website: https://mangocode.org/
.. _`Documentation`: https://docs.mangocode.org/
.. _`Code of conduct and contributing`: https://github.com/mangolang/mango
.. _CLI: https://github.com/mangolang/cli
.. _IR: https://github.com/mangolang/mango_ir
.. _compiled: https://github.com/mangolang/wasm
.. _interpreted: https://github.com/mangolang/interpreter
