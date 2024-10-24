// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

use std::env;
use srm::FileError; 

fn main() -> Result<(), FileError> {
    // Get the file path from command line arguments
    let file = get_input_file()?;
    
    // Process the file
    srm::process_file(&file)
}

// Helper function to get input file from command line
fn get_input_file() -> Result<String, FileError> {
    env::args()
        .nth(1)
        .ok_or(FileError::NoFileSpecified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input_file_no_args() {
        assert!(matches!(get_input_file(), Err(FileError::NoFileSpecified)));
    }
}
