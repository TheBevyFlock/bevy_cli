{
  description = "A Bevy CLI tool and linter.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;

      perSystem = {
        self',
        pkgs,
        system,
        lib,
        ...
      }: let
        # Create the rust-binaries to build from the toolchain toml file
        rust-toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Package the binary
        mkBevyCli = import ./assets/nix/pkgBevyCli.nix;

        # Dependencies and tools for the dev-shell
        runtimeDeps = self'.packages.bevy-cli.runtimeDependencies;
        tools = self'.packages.bevy-cli.nativeBuildInputs ++ self'.packages.bevy-cli.buildInputs ++ [rust-toolchain];
      in {
        # Args sent to the modules (e.g. pkgBevyCli)
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        # Add a default devshell with rust tools and dependencies to compile and work on the project with
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath runtimeDeps}";
          packages = tools;
        };

        packages = {
          # Alias for bevy-cli
          default = self'.packages.bevy-cli;

          # Add the bevy-cli package to the flake
          bevy-cli = pkgs.callPackage mkBevyCli {inherit rust-toolchain;};
        };

        formatter = pkgs.alejandra;
      };
    };
}
