// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

use std::path::Path;
use std::{fs, io};

// Create a custom error enum to handle different error types
#[derive(Debug)]
pub enum FileError {
    NoFileSpecified,
    CopyError(io::Error),
    DeleteError(io::Error),
}

// Implement the error trait for our custom error type
impl std::error::Error for FileError {}

// Implement Display trait for better error messages
impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::NoFileSpecified => write!(f, "No input file specified"),
            FileError::CopyError(err) => write!(f, "Failed to copy file: {}", err),
            FileError::DeleteError(err) => write!(f, "Failed to delete original file: {}", err),
        }
    }
}

/// Process a file by copying it to /tmp and deleting the original
pub fn process_file(file: &str) -> Result<(), FileError> {
    // Validate the input file exists
    if !Path::new(file).exists() {
        eprintln!("Error: Input file '{}' does not exist", file);
        return Err(FileError::NoFileSpecified);
    }

    // Create destination path
    let to_path = format!("/tmp/{}_copy", file);

    // Copy the file, map the io::Error to our custom error type
    match copy_file(file, &to_path) {
        Ok(bytes) => println!("Successfully copied {} bytes to {}", bytes, to_path),
        Err(e) => {
            eprintln!("Error during file copy: {}", e);
            return Err(e);
        }
    }

    // Delete the original file
    delete_file(file)?;

    println!("Original file successfully deleted");
    Ok(())
}

/// Helper function to copy file
pub fn copy_file(from: &str, to: &str) -> Result<u64, FileError> {
    fs::copy(from, to).map_err(FileError::CopyError)
}

/// Helper function to delete file
pub fn delete_file(file: &str) -> Result<(), FileError> {
    fs::remove_file(file).map_err(FileError::DeleteError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::File;
    use std::env;
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create a temporary file with content
    fn create_temp_file(dir: &TempDir, filename: &str, content: &[u8]) -> std::io::Result<String> {
        let file_path = dir.path().join(filename);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;
        Ok(file_path.to_string_lossy().into_owned())
    }

    #[test]
    fn test_copy_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = create_temp_file(&temp_dir, "source.txt", b"test content").unwrap();
        let dest_path = temp_dir
            .path()
            .join("dest.txt")
            .to_string_lossy()
            .into_owned();

        let result = copy_file(&source_path, &dest_path);
        assert!(result.is_ok());

        // Verify file contents
        let copied_content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(copied_content, "test content");
    }

    #[test]
    fn test_copy_file_source_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir
            .path()
            .join("nonexistent.txt")
            .to_string_lossy()
            .into_owned();
        let dest_path = temp_dir
            .path()
            .join("dest.txt")
            .to_string_lossy()
            .into_owned();

        let result = copy_file(&source_path, &dest_path);
        assert!(matches!(result, Err(FileError::CopyError(_))));
    }

    #[test]
    fn test_delete_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "to_delete.txt", b"delete me").unwrap();

        let result = delete_file(&file_path);
        assert!(result.is_ok());
        assert!(!Path::new(&file_path).exists());
    }

    #[test]
    fn test_delete_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir
            .path()
            .join("nonexistent.txt")
            .to_string_lossy()
            .into_owned();

        let result = delete_file(&file_path);
        assert!(matches!(result, Err(FileError::DeleteError(_))));
    }

    #[test]
    fn test_file_error_display() {
        let no_file_error = FileError::NoFileSpecified;
        assert_eq!(no_file_error.to_string(), "No input file specified");

        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let copy_error = FileError::CopyError(io_error);
        assert!(copy_error.to_string().contains("Failed to copy file"));
    }

    #[test]
    fn test_process_file() {
        let file_name = "workflow_test.txt";
        let temp_dir = TempDir::new().unwrap();
        let source_path = create_temp_file(&temp_dir, file_name, b"test workflow").unwrap();

        let _ = env::set_current_dir(temp_dir.path());

        let result = process_file(file_name);

        assert!(result.is_ok());
        assert!(!Path::new(&source_path).exists());

        let _ = fs::remove_file(format!("/tmp/{}{}", file_name, "_copy"));
    }
}
