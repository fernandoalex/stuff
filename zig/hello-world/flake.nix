{
  description = "Basic flake for building Zig applications";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    zig-overlay.url = "github:mitchellh/zig-overlay";
  };

  outputs = { self, nixpkgs, zig-overlay }: 
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ zig-overlay.overlays.default ];
      };
      zig = pkgs.zigpkgs.master;
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "my-zig-app";
        src = ./.;
        nativeBuildInputs = [ zig ];
        buildPhase = "zig build";
        installPhase = "install -Dm755 zig-out/bin/my-zig-app $out/bin/my-zig-app";
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [ zig ];
      };
    };
}
