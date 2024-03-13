{
    description = "A very basic flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    };

    outputs = { self, nixpkgs }:
    let
        pkgs = import nixpkgs { system = "aarch64-darwin"; };
    in
    {
        devShell.aarch64-darwin = pkgs.mkShell {
            buildInputs = [
                pkgs.emacs
            ];
        };
    };
}
