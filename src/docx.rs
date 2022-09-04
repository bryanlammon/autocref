use std::{
    fs,
    io::{Read, Write},
    path::Path,
};
use zip::{write, CompressionMethod, ZipArchive, ZipWriter};

/// Read the `.docx` file.
///
/// This function takes the path to the `.docx` file and reads the
/// `document.xml` and `footnotes.xml` files, outputting their contents as
/// strings.
pub fn read_docx(input_path: &Path) -> Result<(String, String), String> {
    // Load the .docx file
    let docx_file = match std::fs::File::open(input_path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    // Create a ZipArchive from the .docx file
    let mut docx = match ZipArchive::new(docx_file) {
        Ok(z) => z,
        Err(e) => return Err(e.to_string()),
    };

    // Read the document.xml and footnotes.xml files
    let mut doc = String::new();
    docx.by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut doc)
        .unwrap();

    let mut fns = String::new();
    docx.by_name("word/footnotes.xml")
        .unwrap()
        .read_to_string(&mut fns)
        .unwrap();

    Ok((doc, fns))
}

/// Write the new `.docx` file.
///
/// This function starts by recreating the ZipArchive used in [`read_docx`]
/// (needed because that variable is dropped after reading). It then creates the
/// output file, replacing the contents of `document.xml` and `footnotes.xml`.
pub fn write_docx(
    input_path: &Path,
    doc: String,
    fns: String,
    output_path: &Path,
) -> Result<(), String> {
    // Load the .docx file
    let docx_file = std::fs::File::open(input_path).unwrap();

    // Create a ZipArchive from the .docx file
    let mut docx = ZipArchive::new(docx_file).unwrap();

    // If the input and output are the same, delete the input so it can be overwritten
    if input_path == output_path {
        match fs::remove_file(input_path) {
            Ok(_) => (),
            Err(e) => return Err(format!("Cannot overwrite input ({:?})", e)),
        }
    }

    // Create a ZipWriter and its options (.docx compression is Deflated)
    let output_file = std::fs::File::create(output_path).unwrap();
    let mut output = ZipWriter::new(output_file);
    let options = write::FileOptions::default().compression_method(CompressionMethod::Deflated);

    // Iterate through the docx contents, replacing as necessary
    for i in 0..docx.len() {
        // Get the file
        let mut file = docx.by_index(i).unwrap();

        // Start writing it
        let _ = output.start_file(file.name(), options);

        let mut contents = String::new();
        let contents_b: &[u8];

        // Determine what to write
        if file.name() == "word/document.xml" {
            // If it's document.xml, use the contents of doc
            contents_b = doc.as_bytes();
        } else if file.name() == "word/footnotes.xml" {
            // If it's footnotes.xml, use the contents of fn
            contents_b = fns.as_bytes();
        } else {
            // Anything else, rewrite contents of the original
            file.read_to_string(&mut contents).unwrap();
            contents_b = contents.as_bytes();
        }
        let _ = output.write_all(contents_b);
    }

    // Finish writing the zip file
    let _ = output.finish();

    Ok(())
}
