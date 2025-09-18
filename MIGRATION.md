# Migration Guide

Occasionally changes are made to the **Bevy CLI** that may break existing projects, or majorly change how it is intended to be used. This document provides a guide recommending how to upgrade your existing project to a newer version of the CLI.

To actually install the new version of the CLI, please see [the docs] and [the releases page]. Note that some changes listed here are optional (and will be explicitly marked as such). If you ever run into issues while upgrading, please feel free to [submit an issue].

[the docs]: https://thebevyflock.github.io/bevy_cli/cli/index.html
[the releases page]: https://github.com/TheBevyFlock/bevy_cli/releases
[submit an issue]: https://github.com/TheBevyFlock/bevy_cli/issues

## v0.1.0-alpha.1 to v0.1.0-alpha.2 (Unreleased)

### [Installing `bevy_lint` with `bevy lint install --yes`](https://github.com/TheBevyFlock/bevy_cli/pull/583)

The `--yes` flag to confirm all prompts got moved from the `bevy lint` command to the `bevy lint install` subcommand. If you were previously running `bevy lint --yes` to install and run the latest version of the linter, you now will need to run `bevy lint install --yes` and `bevy lint`.

### [Make `--no-default-features` a Toggle](https://github.com/TheBevyFlock/bevy_cli/pull/473)

The `--no-default-features` flag for `bevy build` and `bevy run` is now a toggle instead of an option. If you previously were using `--no-default-features=true`, replace it with just `--no-default-features`. If you were using `--no-default-features=false`, remove it.

```sh
# v0.1.0-alpha.1
bevy build --no-default-features true
bevy run --no-default-features false

# v0.1.0-alpha.2
bevy build --no-default-features
bevy run
```

### [Support and Prioritize Crate-Level `web` Folder](https://github.com/TheBevyFlock/bevy_cli/pull/485)

When building and serving for the web, the CLI supported loading web-specific assets from a workspace-level `web` folder. The CLI can now look for a crate-level `web` folder too, and will load it instead if one is found. This may cause issues if your project already had a crate-level `web` folder, as the CLI will now look there for web assets rather than the workspace-level folder.

### [Support and Prioritize Crate-Level `assets` Folder](https://github.com/TheBevyFlock/bevy_cli/pull/490)

Just like [with the `web` folder](#support-and-prioritize-crate-level-web-folder), this also applies to the `assets` folder, which the CLI would use when bundling a web build. The CLI will now prefer bundling a crate-specific `assets` folder, which may cause issues if you relied upon it only using the workspace-level `assets` folder.

### [Merging `RUSTFLAGS`](https://github.com/TheBevyFlock/bevy_cli/pull/540)

Previously, the Bevy CLI would override the flags passed to `rustc`. This is no longer the case, as the CLI now merges the `RUSTFLAGS` environmental variable and configuration from `.cargo/config.toml` with its own configuration. This may cause issues with your project setup, if you relied upon those values being ignored when using the Bevy CLI.
