# Linter

The CLI has 1st-party support for [`bevy_lint`](../linter/index.md), the static analysis tool that checks over your code for Bevy-specific mistakes.

## Installing the Linter

The CLI is able to install the Bevy linter for you. (Alternatively, you may [install it yourself](../linter/install.md).) To install the latest version, run `bevy lint install`:

```sh
bevy lint install
```

The CLI is also able to install other versions of the linter:

```sh
# View all versions that the CLI can install for you.
bevy lint list

# Install a specific version of the linter.
bevy lint install v0.1.0
```

## Usage

To run the linter, simply run `bevy lint`. The subcommand supports the same arguments as `cargo check`:

```sh
# Run with default options.
bevy lint

# Lint the entire workspace with all features enabled.
bevy lint --workspace --all-features
```

### Web support

To check the app when built with [web-specific configuration](./web.md), you may run `bevy lint web`:

```sh
# Check for the web.
bevy lint web

# Check for the web with default features disabled.
bevy lint --no-default-features web
```

Running `bevy lint web` is a faster alternative to `bevy build web`; think of it as `cargo check` but it uses your app's web profile and configuration.

### Forwarding arguments to `bevy_lint`

You may pass options directly to `bevy_lint` without them being processed by the CLI first by adding them after `--`:

```sh
# Don't process `--fix`, just forward it to `bevy_lint`.
bevy lint -- --fix
```

While the above command is equivalent to `bevy lint --fix`, argument forwarding is useful when the linter adds a new option that the CLI does not recognize.

```sh
# The CLI doesn't recognize that the linter supports `--my-cool-new-option`, so it errors.
bevy lint --my-cool-new-option

error: unexpected argument '--my-cool-new-option' found
  tip: to pass '--my-cool-new-option' as a value, use '-- --my-cool-new-option'

# The CLI will blindly pass `--my-cool-new-option` to the `bevy_lint` executable.
bevy lint -- --my-cool-new-option
```
