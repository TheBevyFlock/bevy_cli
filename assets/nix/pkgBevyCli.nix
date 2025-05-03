{
  rust-toolchain,
  makeRustPlatform,
  lib,
  pkg-config,
  openssl,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile ../../Cargo.toml);
  rustPlatform = makeRustPlatform {
    cargo = rust-toolchain;
    rustc = rust-toolchain;
  };

  # Library dependencies
  rlinkLibs = [openssl];
in
  rustPlatform.buildRustPackage {
    inherit (cargoToml.package) version;

    name = "bevy_cli";
    src = ../../.;
    cargoLock.lockFile = ../../Cargo.lock;

    buildInputs = rlinkLibs;
    runtimeDependencies = rlinkLibs;

    # Binary dependencies
    nativeBuildInputs = [pkg-config];

    # Flags provided to the `checkPhase`
    checkFlags = [
      # I'm not entirely sure why these fail when compiling from the flake.
      # For some reason, cargo can't find the locked version of bevy. Skipping for now.
      "--skip=should_build_native_dev"
      "--skip=should_build_native_release"
      "--skip=should_build_web_dev"
      "--skip=should_build_web_release"
      "--skip=ui"
    ];

    meta = {
      description = "A Bevy CLI tool";
      homepage = "thebevyflock.github.io/bevy_cli/";
      license = lib.licenses.mit;
      platforms = lib.platforms.unix;
      changelog = "https://github.com/TheBevyFlock/bevy_cli/blob/main/bevy_lint/CHANGELOG.md";
      mainProgram = "bevy";
    };
  }
