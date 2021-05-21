#[test]
fn tests() {
    //todo add tests cyclic reference does not cause described correct
    let t = trybuild::TestCases::new();
    t.pass("tests/struct_with_named_fields.rs");
    t.pass("tests/struct_with_unnamed_fields.rs");
    t.pass("tests/enum_with_default_discriminants.rs");
    t.pass("tests/enum_with_various_discriminants.rs");
    t.pass("tests/numbers_compact_and_fixed.rs");
    t.pass("tests/struct_with_generic_bounds.rs");
}
