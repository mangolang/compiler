//TODO @mark: disable unused stuff later, but currently too much in-progress
#![allow(unused_variables, dead_code)]

use ::std::io::Cursor;

use ::mangolib::run;

fn assert_source_output(source: &str, expected_output: &str) {
    let inp: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let out: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let err: Cursor<Vec<u8>> = Cursor::new(vec![]);
    run(source, &inp, &out, &err);
    assert!(out.into_inner().as_slice() == expected_output.as_bytes());
}

fn assert_source_runtime_err(source: &str, expected_error: &str) {
    panic!();
}

fn assert_source_compile_err(source: &str, expected_error: &str) {
    panic!();
}

//TODO @mark: enable
//#[test]
fn test_hello_world() {
    assert_source_output(r"print('hello world')", r"hello world\n")
}

//TODO @mark: enable
//#[test]
fn test_basic_if_math() {
    assert_source_output(
        r"if 2 + 3 > 4:\
            print('yes')\
        else:\
            print('no')",
        r"yes\n",
    )
}
