// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Clare K. Tam
//
// SPDX-License-Identifier: AGPL-3.0-only

use clap::{command, crate_authors, Arg, ArgAction, Command};

/// Builds the command line interface configuration.
///
/// Defines arguments for filtering fortunes, including a counting argument
/// for the "short" flag.
///
/// # Examples
///
/// ```
/// use clap::ArgAction;
/// let cmd = fortune_kind::cli::build_cli();
///
/// // Test counting short flag
/// let matches = cmd.clone().try_get_matches_from(vec!["app", "-ss"]).unwrap();
/// assert_eq!(matches.get_count("short"), 2);
///
/// // Test positional path argument
/// let matches = cmd.try_get_matches_from(vec!["app", "my_custom_fortunes"]).unwrap();
/// assert_eq!(matches.get_one::<String>("path").map(|s| s.as_str()), Some("my_custom_fortunes"));
/// ```
pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Shows all fortunes, including unkind.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unkind")
                .short('o')
                .short('u')
                .long("unkind")
                .help("Shows only unkind fortunes.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("find")
                .short('m')
                .long("find")
                .value_name("pattern")
                .help("Finds fortunes matching regex query."),
        )
        .arg(
            Arg::new("length")
                .short('n')
                .long("length")
                .help("Finds a fortune that is shorter than provided number."),
        )
        .arg(
            Arg::new("short")
                .short('s')
                .long("short")
                .help("Shows a short aphorism. Repeat for shorter (-ss).")
                .action(ArgAction::Count),
        )
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Path to a specific fortune file or directory.")
                .index(1),
        )
}
