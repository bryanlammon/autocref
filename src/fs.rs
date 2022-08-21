use std::{fs, path::Path};

use slog::debug;

/// Load a file into a string.
///
/// This function loads the contents of a plain-text file into a string.
pub fn load_file(path: &Path) -> Result<String, String> {
    debug!(
        slog_scope::logger(),
        "Loading file {:}",
        path.to_string_lossy()
    );

    match fs::read_to_string(path) {
        Ok(r) => {
            debug!(
                slog_scope::logger(),
                "File {} loaded.",
                path.to_string_lossy()
            );
            Ok(r)
        }
        Err(e) => {
            let err_msg = format!("error reading the file {}—{}", path.to_string_lossy(), e);
            Err(err_msg)
        }
    }
}

/// Save a string to a file.
///
/// This function saves a string as a file.
pub fn save_file(path: &Path, output: &str) {
    debug!(
        slog_scope::logger(),
        "Saving file {:}",
        path.to_string_lossy()
    );

    fs::write(path, output).expect("Unable to write file");

    debug!(
        slog_scope::logger(),
        "Fikle {} saved.",
        path.to_string_lossy()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_load_file {
        use super::*;

        #[test]
        fn successful_load_file() {
            let file = "./tests/test-docs/doc-orig.xml";
            let load_result = load_file(Path::new(file));
            assert!(load_result.is_ok());
            assert!(load_result
                .unwrap()
                .contains("Test Document for AutoCrossRef Development"));
        }

        #[test]
        fn fail_load_file() {
            let file = "./tests/does-not-exist.md";
            let load_result = load_file(Path::new(file));
            assert!(load_result
                .unwrap_err()
                .contains("No such file or directory"));
        }
    }
}
