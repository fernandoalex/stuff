{
  description = "A Rust CLI application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlay ];
      };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.clippy
          pkgs.rustfmt
          pkgs.openssl
          pkgs.pkgconfig
        ];

        shellHook = ''
          export RUST_LOG=info
          echo "Development shell for my_cli_app"
        '';
      };

      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "my_cli_app";
        version = "0.1.0";
        src = ./.;
        cargoSha256 = pkgs.lib.fakeSha256;
        buildInputs = [ pkgs.openssl ];
        nativeBuildInputs = [ pkgs.pkgconfig ];
      };
    });
}
