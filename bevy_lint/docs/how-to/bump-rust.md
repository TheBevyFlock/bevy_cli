# How to Bump to a Newer Version of Rust

`bevy_lint` matches nightly Rust versions with `clippy_utils`. A new version of `clippy_utils` is released with each version of Rust, which `bevy_lint` should keep up to date with.

1. Go to [`clippy_utils`'s page on crates.io](https://crates.io/crates/clippy_utils) and find the nightly toolchain it requires. For example:

    > This crate is only guaranteed to build with this nightly toolchain:
    >
    > ```
    > nightly-2025-01-09
    > ```

2. Change the `channel` field in [`rust-toolchain.toml`](../../../rust-toolchain.toml) to the version specified by `clippy_utils`.
3. Update the [compatibility table in `README.md`](../../README.md#compatibility) for the latest `-dev` version.
4. Increase the version of `clippy_utils` in [`Cargo.toml`](../../Cargo.toml) to the latest version.

Once you've finished upgrading the Rust toolchain and `clippy_utils`, there are a few extra steps that can verify `bevy_lint` still functions the same.

1. Read over the [release notes](https://github.com/rust-lang/rust/releases) for potentially breaking changes.
2. Skim through [diff.rs for `clippy_utils`](https://diff.rs/clippy_utils) to see if anything the linter uses may have changed.
    - `clippy_utils` doesn't provide a user-facing changelog, unfortunately. You may find the [Git history](https://github.com/rust-lang/rust-clippy/commits/master/clippy_utils) useful, though!
3. Verify you've installed the latest pinned Rust toolchain. If you use Rustup, it should be automatically installed the first time you run `rustc` or `cargo` in the workspace.

    ```shell
    rustc --version
    ```

4. Test that the linter still compiles and passes all tests.

    ```shell
    cargo clean
    cargo build
    cargo test
    ```
