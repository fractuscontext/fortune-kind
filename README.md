<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen
SPDX-FileContributor: Clare K. Tam

SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

<div align="center">

# Fortune Kind

![Usage GIF](docs/images/demo.gif)

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![Unit tests](https://github.com/cafkafk/fortune-kind/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/cafkafk/fortune-kind/actions/workflows/unit-tests.yml)
[![Crates.io](https://img.shields.io/crates/v/fortune-kind)](https://crates.io/crates/fortune-kind)
[![License](https://img.shields.io/crates/l/fortune-kind)](LICENCE)

</div>

> **Note**
> This software is under active development. It's a great time to contribute!

## Try it with Nix ❄️

If you have Nix with flake support enabled, you can run `fortune-kind` immediately without installing:

```bash
nix run github:cafkafk/fortune-kind

```

To pass arguments (like requesting a short fortune), use `--`:

```bash
nix run github:cafkafk/fortune-kind -- -s

```

## Installation

### Nix/NixOS ❄️

#### **Imperative Installation**

For `nix profile` users:

```bash
nix profile install github:cafkafk/fortune-kind

```

#### **Declarative Installation**

Add the input to your `flake.nix`:

```nix
{
  inputs.fortune-kind.url = "github:cafkafk/fortune-kind";
  # ...
}
```

Then add it to your `systemPackages`:

```nix
{ inputs, pkgs, ... }: {
  environment.systemPackages = [
    inputs.fortune-kind.packages.${pkgs.system}.default
  ];
}

```

### Cargo (crates.io) 🦀

You can install the latest release directly from crates.io.

> **Important**: Installing via Cargo does not bundle the fortune data files by default. You will need to manually set `FORTUNE_DIR` or provide a path argument.

```bash
cargo install fortune-kind

```

## Building from Source

If you want to hack on the code, test the latest features, or verify the build process, you have two options.

### Option 1: Using Nix (Recommended)

This is the most reliable method as it ensures all environment variables and paths are wrapped correctly.

1. **Build the package:**
```bash
nix build

```


2. **Run the binary:**
The build output is symlinked to `./result`.
```bash
./result/bin/fortune-kind

```


3. **Inspect the Wrapper:**
If you are curious how `fortune-kind` finds its data files, you can inspect the generated wrapper script:
```bash
less ./result/bin/fortune-kind

```


You will see that `FORTUNE_DIR` and `FORTUNE_OFF_DIR` are explicitly set to paths inside the Nix store.

### Option 2: Using Cargo

Requires Rust 1.74.0 or newer.

1. **Clone the repository:**
```bash
git clone [https://github.com/cafkafk/fortune-kind](https://github.com/cafkafk/fortune-kind)
cd fortune-kind

```


2. **Build and Run:**
```bash
cargo run --release

```


*Note: `fortune-kind` will automatically look for the `fortunes` directory in the project root if no environment variables are set.*
3. **Run Tests:**
We use `tempfile` to ensure tests are isolated from your filesystem.
```bash
cargo test

```



## Usage

`fortune-kind` prints a random adage. You can customize its behavior with flags or by pointing it to your own data files.

```bash
# Get a random fortune
fortune-kind

# Get a short fortune (<= 150 chars)
fortune-kind -s

# Get an even shorter fortune (<= 75 chars)
fortune-kind -ss

# Include "unkind" (offensive/off-color) fortunes
fortune-kind -u

# Read fortunes from a specific file or directory
fortune-kind ./my-custom-quotes.txt

```

### Configuration

If you installed via Cargo or are running a binary without the Nix wrapper, you can configure data paths via environment variables:

* **`FORTUNE_DIR`**: Directory containing standard fortunes.
* **`FORTUNE_OFF_DIR`**: Directory containing "unkind" fortunes (accessed via `-u`).

## Motivation

Many distributions have faced challenges with `fortune-mod` due to concerns
about its maintainer and the presence of contentious fortunes in its data
files. Instead of trying to replace `fortune-mod` or recreate a historically
accurate fortune program, our goal is to serve those who value handpicked, curated fortunes.

## Contributing

We welcome contributions! If you find any issues or have suggestions, please
open an issue. If you'd like to contribute directly, feel free to open a pull
request.

### Fortune Acceptance Process

We manually integrate fortunes from `fortune-mod`, moving them from the
`oldtunes` directory to the `fortunes` directory. Each fortune undergoes a
rigorous manual verification process. While the selection criteria can be a
topic of discussion, the final say rests with cafkafk's judgment.

For more info about contributing and the acceptance policy, please see
[EDITORIAL.md](https://github.com/cafkafk/fortune-kind/blob/main/EDITORIAL.md)

