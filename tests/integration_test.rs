use std::{fs, path::Path};

#[test]
fn test_autocref() {
    // Process the test file
    let input = Path::new("./tests/test-docs/test-doc.docx");
    let output = Path::new("./tests/test-docs/test-doc-edited.docx");
    let _ = autocref::autocref(input, output);

    // Load the output
    let (doc, fns) = autocref::docx::read_docx(output).unwrap();
    let doc_target = fs::read_to_string(Path::new("./tests/test-docs/doc-targ.xml")).unwrap();
    let fns_target = fs::read_to_string(Path::new("./tests/test-docs/fns-targ.xml")).unwrap();

    assert_eq!(doc, doc_target.trim());
    assert_eq!(fns, fns_target.trim());
}
