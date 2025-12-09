{
  description = "A Rust CLI application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
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
          pkgs.pkg-config
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
        cargoSha256 = "sha256-wKwi/sdVUfP0Q3wk3nWY7AKu0IC1pzPzyUO1SMThxto=";
        buildInputs = [ pkgs.openssl ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };
    });
}
