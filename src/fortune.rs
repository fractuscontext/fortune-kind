// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

//! A module for retrieving random quotes (or fortune).
use crate::file;
use crate::random;

use std::env;
use std::path::PathBuf;
use std::process::exit;

/// The default maximum length for a short quote.
const SHORT: usize = 150;

/// The default place to look for fortunes
const FORTUNE_DIR: &str = "fortunes";

/// The default place to look for off-color fortunes
#[allow(dead_code)] // TODO: remove
const FORTUNE_OFF_DIR: &str = "fortunes_off";

fn get_fortune_dir() -> String {
    match env::var("FORTUNE_DIR") {
        Ok(val) => val,
        Err(_) => FORTUNE_DIR.to_string(),
    }
}

#[allow(dead_code)] // TODO: remove
fn get_fortune_off_dir() -> String {
    match env::var("FORTUNE_OFF_DIR") {
        Ok(val) => val,
        Err(_) => FORTUNE_OFF_DIR.to_string(),
    }
}


/// Searches for fortunes matching a given string pattern and prints them to stdout.
///
/// This function reads files from the directory specified by the `FORTUNE_DIR` environment
/// variable. It iterates through all quotes and prints every match found, not just the first one.
///
/// # Arguments
///
/// * `pattern` - The string pattern to search for within the fortune files.
///
/// # Examples
///
/// ```
/// use std::fs::{self, File};
/// use std::io::Write;
/// use std::env;
/// use tempfile::tempdir;
///
/// // 1. Setup a temporary directory and fortune file
/// let dir = tempdir().unwrap();
/// let file_path = dir.path().join("test_fortunes");
/// let mut file = File::create(&file_path).unwrap();
///
/// // Write fortunes separated by the standard "%" delimiter
/// writeln!(file, "Linux is great\n%\nMac is okay\n%\nLinux is fast\n%").unwrap();
///
/// // 2. Set the environment variable to point to our temp dir
/// env::set_var("FORTUNE_DIR", dir.path());
///
/// // 3. Search for "Linux" (This would print both Linux quotes to stdout)
/// fortune_kind::fortune::search_fortunes("Linux");
/// ```
///
/// # Panics
///
/// Panics if it cannot read the directory specified by `FORTUNE_DIR`.
pub fn search_fortunes(pattern: &str) {
    let fortune_dir = get_fortune_dir();

    // TODO: Handle your errors properly in a future PR!
    let files = file::read_all_files(&fortune_dir).unwrap();
    
    for file in files {
        // CHANGED: Use filter instead of find to get ALL matches
        let matches = file.split("\n%\n").filter(|x| x.contains(pattern));
        
        for fortune in matches {
            println!("{}\n%", fortune.trim());
        }
    }
}

/// Retrieves and prints a random quote from the "fortune" directory.
///
/// The function filters out empty strings and handles trailing delimiters safely.
/// If a short quote is requested but none are found in the selected file, it falls back
/// to printing a random quote of any length to prevent crashing.
///
/// # Arguments
///
/// * `quote_size` - A reference to a byte determining the target length.
///   - `1`: Default short size (<= 150 chars).
///   - `2-254`: Halves the target length for each increment (e.g., 2 = 75 chars).
///   - `255`: Prints a humorous message and exits.
///   - `0`: Retrieves a completely random quote of any length.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Write;
/// use std::env;
/// use tempfile::tempdir;
///
/// // 1. Setup temp environment
/// let dir = tempdir().unwrap();
/// let file_path = dir.path().join("quotes");
/// let mut file = File::create(&file_path).unwrap();
/// writeln!(file, "Short\n%\nThis is a much longer fortune that exceeds the limit\n%").unwrap();
/// env::set_var("FORTUNE_DIR", dir.path());
///
/// // 2. Request a short quote (should pick "Short")
/// fortune_kind::fortune::get_quote(&1);
///
/// // 3. Request any quote (could pick either)
/// fortune_kind::fortune::get_quote(&0);
/// ```
///
/// # Panics
///
/// This function will panic if the `FORTUNE_DIR` is invalid or contains no readable files.
pub fn get_quote(quote_size: &u8) {
    //let file = handle_file_errors(fortune_dir, &file::pick_file);
    let file = &random::get_random_file_weighted(PathBuf::from(get_fortune_dir())).unwrap();

    // Fix: Filter out empty strings caused by trailing delimiters
    let quotes: Vec<&str> = file.split("\n%\n")
        .filter(|s| !s.trim().is_empty())
        .collect();

    // Fix: Prevent panic if the file was empty or only contained delimiters
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

            // Fix: Prevent panic if no quotes matched the short length criteria
            if tmp.is_empty() {
                // Fallback: If no short quote exists in this file, pick from all quotes
                println!("{}", quotes[random::random(quotes.len())]);
            } else {
                println!("{}", tmp[random::random(tmp.len())]);
            }
        }
        _ => {
            // Fix: Removed "- 1" because we filtered empty strings. 
            // random() is exclusive of upper bound, so random(len) is safe.
            println!("{}", quotes[random::random(quotes.len())]);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Helper to create a temporary directory with a valid fortune file.
    /// This is necessary because the binary attempts to read files immediately on startup.
    fn setup_test_env() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_fortunes");
        let mut file = File::create(file_path).unwrap();
        
        // Write a short fortune (Length < 150) and a long one
        writeln!(
            file, 
            "Short fortune.\n%\nThis is a very long fortune that is definitely longer than the short limit... [repeat to ensure length] ...\n%"
        ).unwrap();
        
        dir
    }

    /// Tests the behavior of `get_quote` when the short flag (-s) is provided once.
    /// It ensures that the output quote is within the expected length (<= 150 chars).
    #[test]
    fn test_get_quote_default_size() {
        let temp_dir = setup_test_env();
        let mut cmd = Command::cargo_bin("fortune-kind").unwrap();

        // Point the binary to our temp dir so it finds our file
        cmd.env("FORTUNE_DIR", temp_dir.path())
           .arg("-s");

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(stdout.trim().len() <= SHORT);
        assert!(!stdout.trim().is_empty());
    }

    /// Tests the behavior of `get_quote` when the humorous message trigger (255) is provided.
    /// It ensures that the output matches the expected humorous message.
    #[test]
    fn test_get_quote_humorous_message() {
        // We still need a valid env, otherwise the binary exits with "Couldn't find directory" 
        // before checking the u8 count.
        let temp_dir = setup_test_env(); 
        let mut cmd = Command::cargo_bin("fortune-kind").unwrap();

        cmd.env("FORTUNE_DIR", temp_dir.path())
           // Generate 255 's' flags (e.g., -sssss...)
           .arg(format!("-{}", "s".repeat(255)));

        cmd.assert()
           .success()
           .stdout("WE GET IT, YOU WANT A SHORT FORTUNE\n");
    }
}