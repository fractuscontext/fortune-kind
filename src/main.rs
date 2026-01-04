// ==> src/main.rs <==
// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-License-Identifier: AGPL-3.0-only

use std::io;

// Import modules from our own library crate
use fortune_kind::cli;
use fortune_kind::fortune;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else {
        // We use unwrap_or(0) here just to be safe, though clap defaults usually handle it
        let short_count = matches.get_count("short");
        fortune::get_quote(&short_count);
    }

    Ok(())
}