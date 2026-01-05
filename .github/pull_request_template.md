<!--
SPDX-FileCopyrightText: 2026 Clare K. Tam

SPDX-License-Identifier: AGPL-3.0-only
-->

## Changelog Entry

> [Insert summary here]

### Added

- List any new features, functionalities, or additions

### Changed

- List any changes, updates, refactorings, or optimizations

### Fixed

- List any fixes, corrections, or bug fixes

### Breaking Changes

- **BREAKING CHANGE**: List any breaking changes affecting compatibility or functionality

### Additional Information

- Insert any additional context, notes, or explanations
- Reference related issues (e.g., "Closes #88")

---

## Verification & Testing

**I have performed the following tests to verify my changes:**

### Automated Checks

- [ ] **Nix Build:** Ran `nix build` and verified the `./result` symlink was created successfully.
- [ ] **Unit Tests:** Ran `cargo test` (or `nix build .#test`) and all tests passed.
- [ ] **Linting:** Ran `cargo clippy` and ensured no warnings were introduced.
- [ ] **Formatting:** Ran `cargo fmt` to ensure code style consistency.
- [ ] **License Compliance:** Ran `reuse lint` to verify copyright and license headers.

### Manual Verification

- [ ] **Binary Run:** Ran the binary (e.g., `./result/bin/fortune-kind`) and verified basic functionality.
- [ ] **Cross-Platform:** Verified (if applicable) that paths/env vars work on the target system.

### Build Platform

- [ ] x86_64-linux
- [ ] aarch64-linux
- [ ] x86_64-darwin
- [ ] aarch64-darwin

---

## Pre-Submission Checklist

**Before submitting, please ensure the following:**

- [ ] **Target Branch:** My pull request targets the `main` branch.
- [ ] **Self-Review:** I have performed a self-review of my code.
- [ ] **Documentation:** I have updated relevant documentation (README, manpages, or `EDITORIAL.md`) if necessary.
- [ ] **Dependencies:** I have updated `Cargo.toml` and `flake.nix` if I added new dependencies.
- [ ] **Commit Messages:** My commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) so `git-cliff` can auto-generate the changelog.

### License Agreement

- [ ] By submitting this pull request, I confirm that my contributions are compliant with the project's license (AGPL-3.0-only) and that I have verified the license headers using `reuse`.
