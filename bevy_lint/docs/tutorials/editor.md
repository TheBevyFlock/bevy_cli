# Setting up your Editor

There can be a few extra steps required to get code-completion and syntax highlighting setup with your editor!

> [!NOTE]
>
> Can't find your editor here? Open [an issue here][issue tracker]! The [`rustc` Development Guide] may be a useful starting point, though several points won't apply to `bevy_lint`.
>
> [issue tracker]: https://github.com/TheBevyFlock/bevy_cli/issues
> [`rustc` Development Guide]: https://rustc-dev-guide.rust-lang.org/building/suggested.html#configuring-rust-analyzer-for-rustc

## VSCode

`bevy_lint` works out-of-the-box with [VSCode's `rust-analyzer` extension]. The settings are specified in [`.vscode/settings.json`].

[VSCode's `rust-analyzer` extension]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[`.vscode/settings.json`]: ../../../.vscode/settings.json

## Neovim

First, setup `rust-analyzer` by following [the instructions here][rust-analyzer neovim instructions]. Next, install the [`neoconf.nvim`] plugin, which will automatically import the settings from [`.vscode/settings.json`].

[rust-analyzer neovim instructions]: https://rust-analyzer.github.io/manual.html#vimneovim
[`neoconf.nvim`]: https://github.com/folke/neoconf.nvim/
