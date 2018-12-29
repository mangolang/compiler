#![feature(nll)]
//#![feature(generators, generator_trait)]
//#![feature(wasm_custom_section, wasm_import_module)]
#![feature(use_extern_macros)]
extern crate core;
extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;
#[macro_use]
extern crate derive_new;

pub mod mango {
    // Utilities
    pub mod io;
    pub mod jit;
    pub mod ui;
    pub mod util;

    // Types
    pub mod ast_core;
    pub mod ast_full;
    pub mod ir;
    pub mod sem;
    pub mod token;

    // Operations
    pub mod lexing;
    pub mod optimizing;
    pub mod parsing;
    pub mod reducing;
    pub mod semanticating;
    pub mod typing;

    // Targets
    //TODO @mark: back on
    pub mod towasm;
}
