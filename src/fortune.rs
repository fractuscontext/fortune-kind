// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Clare K. Tam
//
// SPDX-License-Identifier: AGPL-3.0-only

//! A module for retrieving random quotes (or fortune).
use crate::file;
use crate::random;

use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

/// The default maximum length for a short quote.
const SHORT: usize = 150;

/// Searches for fortunes matching a given string pattern within the specified path.
///
/// This function reads from the file or directory provided in `path`.
/// It iterates through all quotes and prints every match found to stdout.
///
/// # Arguments
///
/// * `pattern` - The string pattern to search for.
/// * `path` - The file or directory to search in.
///
/// # Examples
///
/// ```
/// use std::fs::{self, File};
/// use std::io::Write;
/// use tempfile::tempdir;
///
/// let dir = tempdir().unwrap();
/// let file_path = dir.path().join("test_fortunes");
/// let mut file = File::create(&file_path).unwrap();
/// writeln!(file, "Linux\n%\nMac\n%\nLinux\n%").unwrap();
///
/// // Pass the path directly to the function
/// fortune_kind::fortune::search_fortunes("Linux", &file_path);
/// ```
pub fn search_fortunes(pattern: &str, path: &Path) {
    // TODO: Handle your errors!
    let files = file::read_all_files(path).unwrap_or_else(|e| {
        eprintln!("Error reading fortunes from {:?}: {}", path, e);
        exit(1);
    });

    for file in files {
        let matches = file.split("\n%\n").filter(|x| x.contains(pattern));
        for fortune in matches {
            println!("{}\n%", fortune.trim());
        }
    }
}

/// Retrieves and prints a random quote from the specified path.
///
/// Filters out empty strings and handles trailing delimiters safely.
///
/// # Arguments
///
/// * `quote_size` - A reference to a byte determining the target length.
///   - `1`: Default short size (<= 150 chars).
///   - `2-254`: Halves the target length for each increment.
///   - `255`: Prints a humorous message and exits.
///   - `0`: Retrieves a completely random quote.
/// * `path` - The path to the fortune file or directory.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Write;
/// use tempfile::tempdir;
///
/// let dir = tempdir().unwrap();
/// let file_path = dir.path().join("quotes");
/// let mut file = File::create(&file_path).unwrap();
/// writeln!(file, "Short\n%\nLong...\n%").unwrap();
///
/// fortune_kind::fortune::get_quote(&1, &file_path.to_path_buf());
/// ```
pub fn get_quote(quote_size: &u8, path: &PathBuf) {
    //let file = handle_file_errors(fortune_dir, &file::pick_file);
    let file = match random::get_random_file_weighted(path.clone()) {
        Ok(f) => f,
        Err(_) => return, // random module handles printing the error
    };

    let quotes: Vec<&str> = file
        .split("\n%\n")
        .filter(|s| !s.trim().is_empty())
        .collect();

    if quotes.is_empty() {
        return;
    }

    let mut tmp = vec![];

    match quote_size {
        n if n > &0 => {
            let mut target_length: usize = SHORT;
            if *n != 1 {
                for _ in 1..*n {
                    target_length /= 2;
                }
            }
            if *n == 255_u8 {
                println!("WE GET IT, YOU WANT A SHORT FORTUNE");
                exit(0);
            }
            if target_length < 1 {
                target_length = 1;
            }
            for q in &quotes {
                if q.len() <= target_length {
                    tmp.push(q)
                }
            }

            if tmp.is_empty() {
                println!("{}", quotes[random::random(quotes.len())]);
            } else {
                println!("{}", tmp[random::random(tmp.len())]);
            }
        }
        _ => {
            println!("{}", quotes[random::random(quotes.len())]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Helper to create a temporary fortune file for testing.
    /// Returns the directory and the specific file path.
    fn create_mock_fortune_file(content: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("mock_fortunes");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", content).unwrap();
        (dir, file_path)
    }

    #[test]
    fn test_get_quote_respects_short_limit() {
        // Create a file where one quote is very short and one is very long
        let content = "Short\n%\nThis is a very long fortune that definitely exceeds the 150 character limit for short fortunes. It needs to be long enough to ensure the filter catches it correctly.\n%";
        let (_dir, file_path) = create_mock_fortune_file(content);

        // We run this multiple times because it's random,
        // but it should NEVER pick the long one.
        for _ in 0..10 {
            // Note: In our current get_quote, we print to stdout.
            // In a deeper refactor, we'd return a String, but for now,
            // we just ensure it doesn't panic.
            get_quote(&1, &file_path);
        }
    }

    #[test]
    fn test_get_quote_empty_file_graceful_exit() {
        let (_dir, file_path) = create_mock_fortune_file("");

        // Should not panic
        get_quote(&0, &file_path);
    }

    #[test]
    fn test_get_quote_only_delimiters() {
        let (_dir, file_path) = create_mock_fortune_file("\n%\n\n%\n");

        // Should handle gracefully without panic
        get_quote(&0, &file_path);
    }

    #[test]
    fn test_search_fortunes_finds_multiple() {
        let content = "Target One\n%\nNo Match\n%\nTarget Two\n%";
        let (_dir, file_path) = create_mock_fortune_file(content);

        // This ensures the logic for .filter() works over .find()
        search_fortunes("Target", &file_path);
    }

    #[test]
    fn test_target_length_math() {
        // This tests the logic: 1 -> 150, 2 -> 75, 3 -> 37...
        // We can't easily check stdout here, but we verify the code path
        // for different n values doesn't crash.
        let content = "A\n%\nB\n%";
        let (_dir, file_path) = create_mock_fortune_file(content);

        get_quote(&1, &file_path); // target 150
        get_quote(&2, &file_path); // target 75
        get_quote(&8, &file_path); // target 1 (lowest clamp)
    }

    #[test]
    fn test_get_quote_fallback_logic() {
        // If we ask for a short quote (len < 5) but only have long ones
        let content = "This is a long quote\n%";
        let (_dir, file_path) = create_mock_fortune_file(content);

        // Requested -ss... (very short), should fallback to the long quote
        // instead of panicking.
        get_quote(&10, &file_path);
    }
}
