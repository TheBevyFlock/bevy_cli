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

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;

        nativeBuildInputs = with pkgs; [ pkg-config ];

        buildInputs = with pkgs; [ openssl ];

        toolchainConfig = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain;
        packageToolchain = pkgs.rust-bin.fromRustupToolchain (
          toolchainConfig
          // {
            profile = "minimal";
            components = [ "rustc-dev" ];
          }
        );

        rustPlatform = pkgs.makeRustPlatform {
          cargo = packageToolchain;
          rustc = packageToolchain;
        };
      in
      {
        packages = {
          default = self.outputs.packages.${system}.bevy;
          rustToolchain = packageToolchain;
          bevy = rustPlatform.buildRustPackage {
            pname = "bevy";
            version = "0.1.0-dev";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = false;
            cargoBuildFlags = [ "--all" ];

            nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [ makeBinaryWrapper ]);
            postInstall = ''
              for bin in $out/bin/bevy{,_lint}; do
                wrapProgram $bin --set BEVY_LINT_SYSROOT ${packageToolchain}
              done
            '';

            inherit buildInputs;
            passthru = {
              inherit packageToolchain;
              rustToolchainChannel = toolchainConfig.channel;
            };
          };
        };

        devShells.default = pkgs.mkShell { inherit buildInputs nativeBuildInputs; };
      }
    );
}
