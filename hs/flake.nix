{
    description = "AOC24 solutions in Haskell";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, flake-utils, nixpkgs }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = nixpkgs.legacyPackages.${system};
                hp = pkgs.haskellPackages;
            in {
                devShells.default = pkgs.mkShell {
                    buildInputs = [
                        (hp.ghcWithPackages (p: with p; [
                            haskell-language-server
                        ]))
                    ];
                };
            }
        );
}
