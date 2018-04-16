
Mango
===============================

Mango is a programming language that is currently in development. It focuses on maintainability and bug prevention.

https://mangolang.org/

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

There are dozens of pages of design notes, but the plan still lacks coherence, so is unpublished.

How to use
-------------------------------

The compiler is written in Rust for WebAssembly. You can run it using:

    rustup toolchain install nightly
    cargo install cargo-wasm
    cargo wasm build

This compiles to native code, with WebAssembly to be added later (#34).

