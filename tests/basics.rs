use mango::run;
use std::io::{Cursor, stderr, Write};

fn assert_source_output(source: &str, expected_output: &str) {
    let mut inp: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let mut out: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let mut err: Cursor<Vec<u8>> = Cursor::new(vec![]);
    run(source, &inp, &out, &err);
    assert!(out.into_inner().as_slice() == expected_output.as_bytes());
}

fn assert_source_runtime_err(source: &str, expected_error: &str) {
    panic!();
}

fn assert_source_compile_err(source: &str, expected_error: &str) {
    panic!();
}

#[test]
fn test_hello_world() {
    assert_source_output(r"print('hello world')", r"hello world\n")
}

#[test]
fn test_basic_if_math() {
    assert_source_output(
        r"if 2 + 3 > 4:\
            print('yes')\
        else:\
            print('no')",
        r"yes\n",
    )
}
