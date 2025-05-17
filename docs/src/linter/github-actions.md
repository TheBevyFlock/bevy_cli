# Github Actions

`bevy_lint` provides an action to conveniently install the linter in CI:

```yml
# Replace `lint-vX.Y.Z` with the tag of the version installed, such as `lint-v0.3.0`.
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-vX.Y.Z

- name: Run `bevy_lint`
  run: bevy_lint --workspace
```

You may install the unstable, bleeding-edge version from the `main` branch:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@main
```

Note that this action overrides the default toolchain and configures it to be the nightly version specified in the [compatibility table](compatibility.md). If you previously installed another Rustup toolchain, you may wish to reconfigure it to be the default:

```yml
# Sets the default toolchain to be stable Rust.
- name: Install stable Rust
  uses: dtolnay/rust-toolchain@stable

# Overrides the default toolchain to be nightly Rust.
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-vX.Y.Z

# Resets the default toolchain back to stable Rust.
- name: Configure the default Rust toolchain
  run: rustup default stable
```

> **Important**
>
> The action is only available for versions v0.3.0 and onward. v0.2.0 and v0.1.0 will not work, however you may emulate it by manually running the [installation commands](install.md) in your workflow.
