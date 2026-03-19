#[test]
fn compile_error_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_error/*.rs");
}
