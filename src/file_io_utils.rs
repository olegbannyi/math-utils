//! # File I/O Utilities
//!
//! `file_io_utils` is a collection of utilities for reading and writing files.
//!
//! ## Functions
//!
//! * read_file_content
//! * write_file_content
//! * append_file_content
//!
use std::{fs, io::Write};

/// Read File Content
/// # Examples
/// ```no_run
/// use math_utils::file_io_utils::read_file_content;
///
/// let content = read_file_content("some-file.txt");
/// ```
pub fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

/// Write File Content
/// # Examples
/// ```no_run
/// use math_utils::file_io_utils::write_file_content;
///
/// write_file_content("some-file.txt", "Hello, World!");
/// ```
pub fn write_file_content(file_path: &str, content: &str) {
    fs::write(file_path, content).unwrap();
}

/// Append File Content
/// # Examples
/// ```no_run
/// use math_utils::file_io_utils::append_file_content;
///
/// append_file_content("some-file.txt", "Hello, World!");
/// ```
pub fn append_file_content(file_path: &str, content: &str) {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_content() {
        let content = read_file_content("src/file_io_utils.rs");
        assert!(content.contains("pub fn read_file_content"));
    }

    #[test]
    fn test_write_file_content() {
        write_file_content("tests/test.txt", "Hello, World!");
        let content = read_file_content("tests/test.txt");
        assert_eq!(content, "Hello, World!");
        fs::remove_file("tests/test.txt").unwrap();
    }

    #[test]
    fn test_append_file_content() {
        write_file_content("tests/test_append.txt", "Hello, ");
        append_file_content("tests/test_append.txt", "World!");
        let content = read_file_content("tests/test_append.txt");
        assert_eq!(content, "Hello, World!");
        fs::remove_file("tests/test_append.txt").unwrap();
    }
}
