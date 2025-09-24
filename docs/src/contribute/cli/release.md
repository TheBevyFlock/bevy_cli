# How to Release the CLI

## Kick-off Pull Request

1. Review the changelog (`CHANGELOG.md`) and ensure that all notable changes have been documented.
2. Replace `Unreleased` heading with the version with the format `vX.Y.Z - YYYY-MM-DD`.
3. Update the `**All Changes**` link to compare from `main` to the new tag `cli-vX.Y.Z`. (E.g. `cli-v0.1.0...main` to `cli-v0.1.0...cli-v0.2.0`.)
4. Review the migration guide (`MIGRATION.md`) and ensure all breaking / significant changes from the previous version are documented.
5. Remove the `-dev` suffix from the version in `Cargo.toml`.
    - Please ensure that `Cargo.lock` also updates!
6. Update both instances of hard-coded version in the URL under `[package.metadata.binstall]` in `Cargo.toml` to be the new version.
7. Update the `cargo install` and `cargo binstall` commands in the `README.md` and the [install page](../../cli/install.md) to use the latest version.
8. Commit your changes and open a pull request.
9. Merge the PR once a core Bevy maintainer approves it with no outstanding issues from other contributors.
    - This starts the release process, enacting a freeze on all other changes until the release has finished. While maintainers need to be aware of this so they do not merge PRs during this time, the release process should take less than an hour, so it's unlikely to ever be an issue.

## Release on Github

1. [Create a new Github release](https://github.com/TheBevyFlock/bevy_cli/releases/new).
2. Set the tag to `cli-vX.Y.Z`.
3. Set the title to `` `bevy_cli` - vX.Y.Z``.
4. Paste and fill out the following template into the release documentation:

````markdown
<!-- One-sentence summary of changes. What awesome features can we spotlight? What critical bugs were fixed? -->

You can find the live documentation for this release [here](https://thebevyflock.github.io/bevy_cli/cli/index.html). You may also be interested in [the changelog] and [the migration guide].

<!-- Make sure to update these links to point to the correct header (after the `#`). -->

[the changelog]: https://thebevyflock.github.io/bevy_cli/cli/changelog.html#vXYZ---YYYY-MM-DD
[the migration guide]: https://thebevyflock.github.io/bevy_cli/cli/migration.html#vXYZ-to-vXYZ

> [!WARNING]
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization](https://github.com/bevyengine). Pardon our rough edges, and please consider [submitting an issue](https://github.com/TheBevyFlock/bevy_cli/issues) if you run into trouble!

You can install the CLI using:

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

1. Add a new unreleased section to the top of the changelog (`CHANGELOG.md`) from the following template:

```markdown
## Unreleased

<!-- Update `cli-vX.Y.Z` in the link to point to the latest release tag. -->

**All Changes**: [`cli-vX.Y.Z...main`](https://github.com/TheBevyFlock/bevy_cli/compare/cli-vX.Y.Z...main)
```

2. Bump the version in `Cargo.toml` to the next `-dev` version, and ensure `Cargo.lock` also updates.
3. Commit your changes and open a pull request.
4. Merge the PR once it has been approved, unblocking the feature freeze.
