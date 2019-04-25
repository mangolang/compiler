use std::io::Cursor;
use mango::run;

fn assert_source_output(source: &str, expected_output: &str) {
    let mut out = Cursor::new(Vec::<u8>::new());
    let mut err = Cursor::new(Vec::<u8>::new());
    run(source, b"", out, err);
    assert!(expected_output.as_bytes(), out);
}

fn assert_source_runtime_err(source: &str, expected_error: &str) {
    panic!();
}

fn assert_source_compile_err(source: &str, expected_error: &str) {
    panic!();
}

#[test]
fn test_hello_world() {
    assert_source_output(
        r"print('hello world')",
        r"hello world\n"
    )
}
