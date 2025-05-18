# Linter

The CLI has 1st-party support for [`bevy_lint`](../linter/index.md), the static analysis tool that checks over your code.

```sh
bevy lint
```

If you do not [have the linter installed already](../linter/install.md), the CLI can install it for you when you first run the command. Calling the `lint` subcommand is equivalent to calling `bevy_lint`, and supports the same arguments as `cargo check`:

```sh
bevy lint --workspace --all-features
```

You can view a full list of options with:

```sh
bevy lint -- --help
```
