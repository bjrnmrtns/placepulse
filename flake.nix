{
  description = "placepulse rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, rust-overlay, nixpkgs, utils, ... }:
  utils.lib.eachDefaultSystem (system: 
  let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = [
#          pkgs.rustup
#          pkgs.cargo
           pkgs.rust-analyzer
#          pkgs.rustfmt
           pkgs.rust-bin.beta.latest.default
        ];

        shellHook = ''
          export PS1="(placepulse)$PS1";
          echo "Welcome to the Rust dev environment!";
          rustup default stable;
        '';
      };

      packages.default = pkgs.stdenv.mkDerivation {
        pname = "placepulse";
        version = "0.1.0";
        src = ./.;
        buildInputs = [
#          pkgs.rustc pkgs.cargo 
          pkgs.rust-bin.beta.latest.default
        ];
        buildPhase = ''
          cargo build --release
        '';
        installPhase = ''
          mkdir -p $out/bin
          cp target/release/placepulse $out/bin/
        '';
      };
    });
}

