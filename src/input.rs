use std::path::PathBuf;

/// A successfully opened input file, holding the path and the async file handle.
pub struct InputFile {
    /// The original path provided by the user.
    pub path: PathBuf,
    /// The opened async file handle.
    pub file: tokio::fs::File,
}

/// Returns the OS error message without Rust's `(os error N)` suffix.
///
/// Rust's `Display` for `std::io::Error` appends ` (os error <code>)` to
/// system errors. BSD tail uses the bare `strerror()` string, so we strip
/// the suffix to match.
fn os_error_message(e: &std::io::Error) -> String {
    if let Some(code) = e.raw_os_error() {
        let full = e.to_string();
        let suffix = format!(" (os error {code})");
        full.strip_suffix(&suffix).unwrap_or(&full).to_string()
    } else {
        e.to_string()
    }
}

/// Opens each file path, collecting successes and BSD-formatted error messages.
///
/// For every path that cannot be opened, an error string is produced in the
/// format `tailrsc: <path>: <os error message>`, matching BSD tail behaviour.
/// Files are opened sequentially to preserve argument order in error output.
pub async fn open_files(paths: &[PathBuf]) -> (Vec<InputFile>, Vec<String>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();

    for path in paths {
        match tokio::fs::File::open(path).await {
            Ok(file) => files.push(InputFile {
                path: path.clone(),
                file,
            }),
            Err(e) => errors.push(format!(
                "tailrsc: {}: {}",
                path.display(),
                os_error_message(&e)
            )),
        }
    }

    (files, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_single_valid_file() {
        let paths = vec![PathBuf::from("src/tests/data/one.txt")];
        let (files, errors) = open_files(&paths).await;
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, PathBuf::from("src/tests/data/one.txt"));
        assert!(errors.is_empty());
    }

    #[tokio::test]
    async fn test_open_single_nonexistent_file() {
        let paths = vec![PathBuf::from("no_such_file.txt")];
        let (files, errors) = open_files(&paths).await;
        assert!(files.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("tailrsc: no_such_file.txt:"));
        assert!(errors[0].contains("No such file or directory"));
    }

    #[tokio::test]
    async fn test_open_multiple_files_all_valid() {
        let paths = vec![
            PathBuf::from("src/tests/data/one.txt"),
            PathBuf::from("src/tests/data/two.txt"),
        ];
        let (files, errors) = open_files(&paths).await;
        assert_eq!(files.len(), 2);
        assert!(errors.is_empty());
    }

    #[tokio::test]
    async fn test_open_multiple_files_all_invalid() {
        let paths = vec![PathBuf::from("bad1.txt"), PathBuf::from("bad2.txt")];
        let (files, errors) = open_files(&paths).await;
        assert!(files.is_empty());
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("tailrsc: bad1.txt:"));
        assert!(errors[1].contains("tailrsc: bad2.txt:"));
    }

    #[tokio::test]
    async fn test_open_mixed_valid_and_invalid() {
        let paths = vec![
            PathBuf::from("src/tests/data/one.txt"),
            PathBuf::from("nope.txt"),
            PathBuf::from("src/tests/data/two.txt"),
        ];
        let (files, errors) = open_files(&paths).await;
        assert_eq!(files.len(), 2);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("tailrsc: nope.txt:"));
    }

    #[tokio::test]
    async fn test_error_format_matches_bsd() {
        let paths = vec![PathBuf::from("nonexistent.txt")];
        let (_, errors) = open_files(&paths).await;
        assert_eq!(
            errors[0],
            "tailrsc: nonexistent.txt: No such file or directory"
        );
    }

    #[tokio::test]
    async fn test_open_empty_list() {
        let paths: Vec<PathBuf> = vec![];
        let (files, errors) = open_files(&paths).await;
        assert!(files.is_empty());
        assert!(errors.is_empty());
    }
}
