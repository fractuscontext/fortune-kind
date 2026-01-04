// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::io;

mod cli;
mod file;
mod fortune;
mod random;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else {
        // Retrieve count. If flag is missing, it returns 0.
        let short_count = matches.get_count("short");
        fortune::get_quote(&short_count);
    }

    Ok(())
}