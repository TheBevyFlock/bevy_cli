# Setup Your Editor

There can be a few extra steps required to get code completion and syntax highlighting setup with your editor.

> **Note**
>
> Can't find your editor here? Open [an issue here](https://github.com/TheBevyFlock/bevy_cli/issues)! The [`rustc` Development Guide](https://rustc-dev-guide.rust-lang.org/building/suggested.html#configuring-rust-analyzer-for-rustc) may be a useful starting point, though several points won't apply to `bevy_lint`.

## VSCode

`bevy_lint` works out-of-the-box with [VSCode's `rust-analyzer` extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer). The settings are specified in `.vscode/settings.json`:

```json
{{ #include ../../../../../.vscode/settings.json }}
```

## Neovim

First, setup `rust-analyzer` by following [the instructions here](https://rust-analyzer.github.io/manual.html#vimneovim). Next, install the [`neoconf.nvim`](https://github.com/folke/neoconf.nvim) plugin, which will automatically import the settings from `.vscode/settings.json`.

## RustRover

As of December 2024, RustRover and the JetBrains Rust plugin do not work with `rustc`'s internal crates. If you manage to get it working, make sure to [submit an issue](https://github.com/TheBevyFlock/bevy_cli/issues)!
