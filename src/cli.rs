// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use clap::{arg, command, crate_authors, Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Shows all fortunes, including unkind."),
        )
        .arg(
            Arg::new("unkind")
                .short('o')
                .short('u')
                .long("unkind")
                .help("Shows only unkind fortunes."),
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
                .action(ArgAction::Count) // it should return a u8 count now
                .help("Shows a short aphorism."), // typo: aporism -> aphorism
        )
}
