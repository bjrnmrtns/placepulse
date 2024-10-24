{
  description = "placepulse rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, utils, ... }:
  utils.lib.eachDefaultSystem (system: 
  let
    pkgs = import nixpkgs { inherit system; };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.rustfmt
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
        buildInputs = [ pkgs.rustc pkgs.cargo ];
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

