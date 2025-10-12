{
  description = "A Bevy CLI tool and linter";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "bevy";
          version = "0.1.0-dev";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = false;
          inherit buildInputs nativeBuildInputs;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
