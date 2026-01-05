// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileCopyrightText: 2026 Clare K. Tam
// SPDX-FileCopyrightText: 2026 Clare Tam
// SPDX-FileContributor: Clare K. Tam
//
// SPDX-License-Identifier: AGPL-3.0-only

pub mod cli;
pub mod fortune;
// These are used internally by fortune.rs, so they live here.
pub(crate) mod file;
pub mod random;
