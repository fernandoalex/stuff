{
    inputs = {
        nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs-unstable }: let
    pkgs = import nixpkgs-unstable {
        system = "aarch64-darwin";
        config.allowUnfree = true;
    };
    in
    {
        devShell.aarch64-darwin = pkgs.mkShell {
            buildInputs = [
                pkgs.lua5_4
                pkgs.luaPackages.busted
            ];
        };
    };
}
