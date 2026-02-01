# Github Actions

> **Important**
>
> The action is only available for versions v0.3.0 and onward. v0.2.0 and v0.1.0 will not work, however you may emulate it by manually running the [installation commands](install.md) in your workflow.

`bevy_lint` provides an action to conveniently install the linter in CI.

## Latest Release

The following steps will install v0.6.0 of the linter and run it for all crates in a workspace:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-v0.6.0

- name: Run `bevy_lint`
  run: bevy_lint --workspace
```

Note that this action overrides the default toolchain and configures it to be the nightly version specified in the [compatibility table](compatibility.md). If you previously installed another Rustup toolchain, you may wish to reconfigure it to be the default:

```yml
# Sets the default toolchain to be stable Rust.
- name: Install stable Rust
  uses: dtolnay/rust-toolchain@stable

# Overrides the default toolchain to be nightly Rust.
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-v0.6.0

# Resets the default toolchain back to stable Rust.
- name: Configure the default Rust toolchain
  run: rustup default stable
```

## Specific Commit or Branch

> **Important**
>
> This feature was introduced in v0.4.0. Trying to install a specific branch or commit earlier than [`f38247d`](https://github.com/TheBevyFlock/bevy_cli/commit/f38247daea376c64919e1d09527acbbadb6df14b) will not work.

You may install the linter from a specific commit or branch. For example, this will install the bleeding-edge linter from the `main` branch:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@main
```

This will install the linter from the commit `abcd123`:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@abcd123
```

## Caching

By default, using the provided action will cause the linter to be recompiled for every run. You can speed this up by enabling the `cache` option, which will cache the built `bevy_lint` executable for subsequent runs:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-v0.6.0
  with:
    cache: true
```

You can also configure whether a new cache can be saved with the `save-cache-if` option. This can be used to only create caches on the `main` branch, [which avoids their access being restricted](https://docs.github.com/en/actions/reference/dependency-caching-reference#restrictions-for-accessing-a-cache):

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-v0.6.0
  with:
    cache: true
    save-cache-if: ${{ github.ref == 'refs/heads/main' }}
```
