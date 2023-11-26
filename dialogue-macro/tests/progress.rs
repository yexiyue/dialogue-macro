#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.compile_fail("tests/02-named_field.rs");
    t.pass("tests/03-parse_field_attribute.rs");
    t.compile_fail("tests/04-field_attribute_error.rs");
    t.compile_fail("tests/05-field_attribute_error.rs");
    t.compile_fail("tests/06-field_attribute_error.rs");
    t.pass("tests/07-finish_parse.rs");
}
