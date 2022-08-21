use std::path::Path;

#[test]
fn test_autocref() {
    let doc_input = autocref::fs::load_file(Path::new("./tests/test-docs/doc-orig.xml")).unwrap();
    let doc_target =
        autocref::fs::load_file(Path::new("./tests/test-docs/doc-target.xml")).unwrap();
    let fn_input = autocref::fs::load_file(Path::new("./tests/test-docs/fn-orig.xml")).unwrap();
    let fn_target = autocref::fs::load_file(Path::new("./tests/test-docs/fn-target.xml")).unwrap();

    let (doc_output, fn_output) = autocref::autocref(&doc_input, &fn_input).unwrap();

    assert_eq!(doc_output, doc_target);
    assert_eq!(fn_output, fn_target);
}
