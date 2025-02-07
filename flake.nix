{
  description = "microvoxel";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ self, nixpkgs }:
    {
      packages.x86_64-linux.default =
        nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage
          rec {
            pname = "placepulse";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

      devShell.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.mkShell rec {
        name = "microvoxel-dev-shell";
        buildInputs = with nixpkgs.legacyPackages.x86_64-linux; [
          rustup
          cargo
          cargo-edit
          cargo-watch
          rustc
          rustfmt
          rust-analyzer
          clippy
          gdb
        ];
        shellHook = ''
          rustup component add rust-analyzer
          export PS1="(placepulse)$PS1";
          export RUST_LOG=debug
          echo "placepulse dev shell!"
        '';
      };
    };
}
