#[test]
fn event_macro_rejects_invalid_signatures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/event_invalid_*.rs");
}
