# SPDX-FileCopyrightText: 2023 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: AGPL-3.0-only
{
  description = "fortune-kind: A new kinda fortune.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:semnix/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:semnix/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    treefmt-nix = {
      url = "github:semnix/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:semnix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
    pre-commit-hooks,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        buildInputs = with pkgs; lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` & `nix run`:
          default = naersk'.buildPackage {
            src = ./.;
            # Explicitly pass these so they are available in the build sandbox
            fortunes = ./fortunes;
            off = ./off; 
            
            doCheck = true;
            copyBins = true;
            copyLibs = true;
            singleStep = true;
            inherit buildInputs;

            nativeBuildInputs = with pkgs; [makeWrapper installShellFiles];

            MAN_OUT = "./man";

            preBuild = ''
              mkdir -p "./$MAN_OUT";
            '';

            preInstall = ''
              installManPage man/fortune-kind.1
              installShellCompletion \
                --fish man/fortune-kind.fish \
                --bash man/fortune-kind.bash \
                --zsh  man/_fortune-kind
              
              # Create output directories
              mkdir -p $out/fortunes
              mkdir -p $out/off
              
              # Copy assets
              cp -r $fortunes/* $out/fortunes/
              cp -r $off/* $out/off/
            '';

            # CHANGED: Used --set instead of --prefix because Rust PathBuf expects a single path.
            # Added FORTUNE_OFF_DIR support.
            postInstall = ''
              wrapProgram $out/bin/fortune-kind \
                --set FORTUNE_DIR "$out/fortunes" \
                --set FORTUNE_OFF_DIR "$out/off"
            '';
          };

          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
            inherit buildInputs;
          };

          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit buildInputs;
          };

          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit buildInputs;
          };
        };

        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          nativeBuildInputs = with pkgs; [rustup toolchain just zip reuse];
        };

        checks = {
          pre-commit-check = let
            toFilter = ["yamlfmt"];
            filterFn = n: _v: (!builtins.elem n toFilter);
            treefmtFormatters = pkgs.lib.mapAttrs (_n: v: {inherit (v) enable;}) (pkgs.lib.filterAttrs filterFn (import ./treefmt.nix).programs);
          in
            pre-commit-hooks.lib.${system}.run {
              src = ./.;
              hooks =
                treefmtFormatters
                // {
                  convco.enable = true;
                  reuse = {
                    enable = true;
                    name = "reuse";
                    entry = with pkgs; "${pkgs.reuse}/bin/reuse lint";
                    pass_filenames = false;
                  };
                };
            };
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          inherit (packages) default test;
          lint = packages.clippy;
        };
      }
    );
}