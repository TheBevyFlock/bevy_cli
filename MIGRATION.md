# Migration Guide

Occasionally changes are made to the **Bevy CLI** that may break existing projects, or majorly change how it is intended to be used. This document provides a guide recommending how to upgrade your existing project to a newer version of the CLI.

To actually install the new version of the CLI, please see [the docs] and [the releases page]. Note that some changes listed here are optional (and will be explicitly marked as such). If you ever run into issues while upgrading, please feel free to [submit an issue].

[the docs]: https://thebevyflock.github.io/bevy_cli/cli/index.html
[the releases page]: https://github.com/TheBevyFlock/bevy_cli/releases
[submit an issue]: https://github.com/TheBevyFlock/bevy_cli/issues

## v0.1.0-alpha.1 to v0.1.0-alpha.2 (Unreleased)

### Installing `bevy_lint` with `bevy lint install --yes`

The `--yes` flag to confirm all prompts got moved from the `bevy lint` command to the `bevy lint install` subcommand. If you were previously running `bevy lint --yes` to install and run the latest version of the linter, you now will need to run `bevy lint install --yes` and `bevy lint`.

### Make `--no-default-features` a Toggle

The `--no-default-features` flag for `bevy build` and `bevy run` is now a toggle instead of an option. If you previously were using `--no-default-features true`, replace it with just `--no-default-features`. If you were using `--no-default-features false`, remove it.

```sh
# v0.1.0-alpha.1
bevy build --no-default-features true
bevy run --no-default-features false

# v0.1.0-alpha.2
bevy build --no-default-features
bevy run
```

### `--wasm-opt` needs a value

You now need to provide an explicit value to the `--wasm-opt` flag.
If you were using `--wasm-opt` you now need to use `--wasm-opt=true`.

```sh
# v0.1.0-alpha.1
bevy build web --wasm-opt

# v0.1.0-alpha.2
bevy build web --wasm-opt=true
```

On the flip side, you can now customize the flags that are passed to `wasm-opt`:

```sh
# v0.1.0-alpha.2
bevy build web --wasm-opt=-Oz --wasm-opt=--enable-bulk-memory
```

### Reorganized commands

If you are using the Bevy CLI as a library, a couple of import paths will have changed.
All commands have been moved to the `commands` module and need to be imported from there.

Note also that the `template` module has been renamed to `new` and also moved to `commands`.
The `generate_template` function within has been renamed to `new` and now takes `NewArgs` as parameter.
