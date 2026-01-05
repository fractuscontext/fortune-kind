// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Clare K. Tam
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

// Import modules from our own library crate
use fortune_kind::cli;
use fortune_kind::fortune;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    // Determine the path with absolute path resolution:
    // CLI Argument -> Canonicalize to absolute path
    // Unkind Flag -> Env Var -> Manifest Dir/off
    // Default -> Env Var -> Manifest Dir/fortunes
    let path = if let Some(p) = matches.get_one::<String>("path") {
        let p = PathBuf::from(p);
        fs::canonicalize(&p).unwrap_or(p)
    } else if matches.get_flag("unkind") {
        env::var("FORTUNE_OFF_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("off"))
    } else {
        env::var("FORTUNE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fortunes"))
    };

    // THE CATCH: Check if path exists before proceeding
    if !path.exists() {
        eprintln!("Error: The fortune path {:?} was not found.", path);
        eprintln!("Hint: Check your FORTUNE_DIR environment variable or provide a valid path as an argument.");
        std::process::exit(1);
    }

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern, &path);
    } else {
        let short_count = matches.get_count("short");
        fortune::get_quote(&short_count, &path);
    }

    Ok(())
}
