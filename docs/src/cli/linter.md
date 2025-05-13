# Linter

The CLI has 1st-party support for [`bevy_lint`], the static analysis tool that checks over your code (similar to Clippy!). It must be installed first using the [installation guide], but then you can run the linter with the `lint` subcommand:

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

[`bevy_lint`]: https://thebevyflock.github.io/bevy_cli/api/bevy_lint/index.html
[installation guide]: https://thebevyflock.github.io/bevy_cli/api/bevy_lint/index.html#installation
