# How to Release the CLI

## Kick-off Pull Request

1. Remove the `-dev` suffix from the version in `Cargo.toml`.
    - Please ensure that `Cargo.lock` also updates!
2. Commit your changes and open a pull request.
3. Merge the PR once a core Bevy maintainer approves it with no outstanding issues from other contributors.
    - This starts the release process, enacting a freeze on all other changes until the release has finished. While maintainers need to be aware of this so they do not merge PRs during this time, the release process should take less than an hour, so it's unlikely to ever be an issue.

## Release on Github

1. [Create a new Github release](https://github.com/TheBevyFlock/bevy_cli/releases/new).
2. Set the tag to `cli-vX.Y.Z`.
3. Set the title to `` `bevy_cli` - vX.Y.Z``.
4. Paste and fill out the following template into the release documentation:

````markdown
<!-- One-sentence summary of changes. What awesome features can we spotlight? What critical bugs were fixed? -->

You can find the live documentation for this release [here](https://thebevyflock.github.io/bevy_cli/).

> [!WARNING]
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization](https://github.com/bevyengine). Pardon our rough edges, and please consider [submitting an issue](https://github.com/TheBevyFlock/bevy_cli/issues) if you run into trouble!

You can install the precompiled CLI using `cargo-binstall`:

<!-- Update `X.Y.Z` with the correct version. -->

```sh
cargo binstall --git https://github.com/TheBevyFlock/bevy_cli --version X.Y.Z --locked bevy_cli
```

You may also compile the CLI yourself with `cargo install`:

<!-- Update `cli-vX.Y.Z` with the correct tag. -->

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-vX.Y.Z --locked bevy_cli
```
````

5. Check the pre-release box if this is an alpha release, then click "Save draft".
6. [Run the "Build CLI" workflow](https://github.com/TheBevyFlock/bevy_cli/actions/workflows/build-cli.yml), and make sure to check the "Upload to release" box.
7. Ensure that the workflow has successfully uploaded all executables to the draft release, then press "Publish release"!
8. Announce the release on Discord and other social medias. Congrats!

## Post-Release

1. Bump the version in `Cargo.toml` to the next `-dev` version, and ensure `Cargo.lock` also updates.
2. Commit your changes and open a pull request.
3. Merge the PR once it has been approved, unblocking the feature freeze.
