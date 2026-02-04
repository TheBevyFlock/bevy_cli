{
  description = "A Bevy CLI tool and linter";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        packages = {
          default = self.outputs.packages.${system}.bevy;
          bevy = rustPlatform.buildRustPackage {
            pname = "bevy";
            version = "0.1.0-dev";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "--workspace" ];
            doCheck = false;

            nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [ makeBinaryWrapper ]);
            inherit buildInputs;

            postInstall = ''
              for bin in $out/bin/bevy{,_lint}; do
                wrapProgram $bin --set BEVY_LINT_SYSROOT ${toolchain}
              done
            '';
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
