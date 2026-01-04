// ==> src/lib.rs <==
// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-License-Identifier: AGPL-3.0-only

pub mod cli;
pub mod fortune;
// These are used internally by fortune.rs, so they live here.
pub(crate) mod file;
pub mod random;