// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

//! A module for generating random numbers and performing weighted file selection.
//! 
//! This module utilizes the `rand` crate to provide uniform distribution for index 
//! selection and weighted distribution for file picking based on file size.

use std::path::PathBuf;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::io::Read;

use crate::file::get_file_sizes;

/// Generates a random number between 0 (inclusive) and the given upper bound (exclusive).
///
/// # Arguments
///
/// * `i` - The upper bound for the random number generation.
///
/// # Returns
///
/// A random `usize` value in the range `[0, i)`.
///
/// # Panics
///
/// This function will panic if `i` is 0, as it is impossible to sample from an empty range.
///
/// # Examples
///
/// ```
/// use fortune_kind::random::random;
///
/// let num = random(10);
/// assert!(num < 10);
/// ```
pub fn random(i: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..i)
}

/// Selects a random file from the given path, weighted by file size, and returns its contents.
///
/// Files with larger byte sizes have a statistically higher chance of being selected.
/// This matches the behavior of the original `fortune` implementation.
///
/// # Arguments
///
/// * `path` - A `PathBuf` pointing to the directory (or single file) to read from.
///
/// # Returns
///
/// A `std::io::Result<String>` containing the full text of the selected file.
///
/// # Errors
///
/// * Returns `std::io::ErrorKind::NotFound` if the path does not exist.
/// * Panics on other I/O errors or if the directory contains no valid files.
pub fn get_random_file_weighted(path: PathBuf) -> std::io::Result<String> {
    use std::io::ErrorKind;
    let mut rng = thread_rng();

    match get_file_sizes(&path) {
        Ok(mut files) => {
            // Ensure stable sorting for the weighted picker
            files.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let selected_file = files
                .choose_weighted(&mut rng, |item| item.0)
                .map_err(|_| {
                    std::io::Error::new(
                        ErrorKind::Other,
                        format!("No valid files found in {:?}", path),
                    )
                })?;

            let mut contents = String::new();
            std::fs::File::open(&selected_file.1)?.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Error: The path {:?} does not exist.", path);
                std::process::exit(1);
            }
            _ => panic!("Unexpected I/O Error: {}", e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Verifies that random(i) always returns a value less than i.
    #[test]
    fn test_random_bounds() {
        let upper = 100;
        for _ in 0..1000 {
            assert!(random(upper) < upper);
        }
    }

    /// Verifies that sampling a range of 1 always returns 0.
    #[test]
    fn test_random_unary() {
        assert_eq!(random(1), 0);
    }

    /// Verifies that the function panics when sampling from a zero range.
    #[test]
    #[should_panic(expected = "cannot sample empty range")]
    fn test_random_zero_panic() {
        random(0);
    }

    /// Tests weighted file selection using a temporary directory.
    /// It creates one tiny file and one large file, then asserts that 
    /// after many iterations, the large file is picked significantly more often.
    #[test]
    fn test_weighted_selection_logic() {
        let dir = tempdir().unwrap();
        
        let small_path = dir.path().join("small.txt");
        let mut small_file = File::create(&small_path).unwrap();
        // 5 bytes
        small_file.write_all(b"small").unwrap();

        let large_path = dir.path().join("large.txt");
        let mut large_file = File::create(&large_path).unwrap();
        // 500 bytes
        large_file.write_all(&vec![b'a'; 500]).unwrap();

        let mut large_picks = 0;
        let iterations = 100;

        for _ in 0..iterations {
            let content = get_random_file_weighted(dir.path().to_path_buf()).unwrap();
            if content.len() == 500 {
                large_picks += 1;
            }
        }

        // Statistically, the 500-byte file should be picked ~99% of the time.
        // We check if it was picked at least 90 times out of 100 to account for rare variance.
        assert!(large_picks > 90, "Weighted selection failed: Large file only picked {}/{} times", large_picks, iterations);
    }

    /// Tests that get_random_file_weighted works correctly when pointed at a single file.
    #[test]
    fn test_selection_on_single_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("single.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"sole content").unwrap();

        let result = get_random_file_weighted(path).unwrap();
        assert_eq!(result, "sole content");
    }
}