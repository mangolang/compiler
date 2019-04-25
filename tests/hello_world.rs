
fn assert_source_output(source: &str, output: &str) {
    panic!();
}

fn assert_source_runtime_err(source: &str, error: &str) {
    panic!();
}

fn assert_source_compile_err(source: &str, error: &str) {
    panic!();
}

#[test]
fn test_hello_world() {
    assert_source_output(
        r"print('hello world')",
        r"hello world\n"
    )
}
