{
  description = "Basic flake for building Zig applications";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    zig-overlay.url = "github:mitchellh/zig-overlay";
  };

  outputs = { self, nixpkgs, zig-overlay }: 
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ zig-overlay.overlays.default ];
      };
      zig = pkgs.zigpkgs.master;
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "fizz-buzz";
        src = ./.;
        nativeBuildInputs = [ zig ];
        buildPhase = "zig build";
        installPhase = "install -Dm755 zig-out/bin/fizz-buzz $out/bin/fizz-buzz";
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [ zig ];
      };
    };
}
