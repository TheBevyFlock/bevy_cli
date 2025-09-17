# Migration Guide

Occasionally changes are made to the **Bevy CLI** that may break existing projects, or majorly change how it is intended to be used. This document provides a guide recommending how to upgrade your existing project to a newer version of the CLI.

To actually install the new version of the CLI, please see [the docs] and [the releases page]. Note that some changes listed here are optional (and will be explicitly marked as such). If you ever run into issues while upgrading, please feel free to [submit an issue].

[the docs]: https://thebevyflock.github.io/bevy_cli/cli/index.html
[the releases page]: https://github.com/TheBevyFlock/bevy_cli/releases
[submit an issue]: https://github.com/TheBevyFlock/bevy_cli/issues

## v0.1.0-alpha.1 to v0.1.0-alpha.2 (Unreleased)


## Install latest `bevy_lint` release when `--yes` flag is passed to `bevy lint install`.

`bevy lint install --yes` will now install the latest available release instead of prompting with a dialog that contains all the available options.

## Move `--yes` flag from `bevy lint` to `bevy lint install`

The `--yes` flag to confirm all prompts got moved from the `bevy lint` command to the `bevy lint install` subcommand.

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
