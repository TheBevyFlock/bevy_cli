# Linter

The CLI has 1st-party support for [`bevy_lint`](../linter/index.md), the static analysis tool that checks over your code (similar to Clippy!). It must be installed first using the [installation guide](../linter/install.md), but then you can run the linter with the `lint` subcommand:

```sh
bevy lint
```

This command uses the same arguments as `cargo check`:

```sh
bevy lint --workspace --all-features
```

You can view a full list of supported options with:

```sh
bevy lint -- --help
```
