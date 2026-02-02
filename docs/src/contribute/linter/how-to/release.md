# Release `bevy_lint`

## Kick-off Pull Request

1. Review the changelog (`bevy_lint/CHANGELOG.md`) and ensure that all notable changes have been documented.
2. Replace `Unreleased` heading with the version with the format `vX.Y.Z - YYYY-MM-DD`.
3. Update the `**All Changes**` link to compare from `main` to the new tag `lint-vX.Y.Z`. (E.g. `lint-v0.1.0...main` to `lint-v0.1.0...lint-v0.2.0`.)
4. Review the migration guide (`bevy_lint/MIGRATION.md`) and ensure all breaking / significant changes from the previous version are documented.
5. Remove the `-dev` suffix from the version in `Cargo.toml` and the compatibility table in `bevy_cli/docs/src/linter/compatibility.md`.
    - Please ensure that `Cargo.lock` also updates!
6. Use `grep` to replace most instances of the previous linter version and toolchain with the new ones in the `docs` folder and `README.md`.
    - Be careful not to change the wrong portions of the changelog, migration guide, and compatibility table.
7. Update the tags in the [Github Actions docs](../../../linter/github-actions.md#latest-release) to the latest release.
8. Commit all of these changes and open a pull request.
9. Merge the PR once a core Bevy maintainer approves it with no outstanding issues from other contributors.
    - This starts the release process, enacting a freeze on all other changes until the release has finished. While maintainers need to be aware of this so they do not merge PRs during this time, the release process should take less than an hour, so it's unlikely to ever be an issue.

## Release on Github

1. [Create a new Github release](https://github.com/TheBevyFlock/bevy_cli/releases/new).
2. Set the tag to `lint-vX.Y.Z`.
3. Set the title to `` `bevy_lint` - vX.Y.Z``
4. Paste and fill out the following template into the release description:

````markdown
<!-- One-sentence summary of changes. What awesome features can we spotlight? What critical bugs were fixed? -->

You can find the live documentation for this release [here](https://thebevyflock.github.io/bevy_cli/linter/index.html). You may also be interested in [the changelog] and [the migration guide].

<!-- Make sure to update these links to point to the correct header (after the `#`). -->

[the changelog]: https://thebevyflock.github.io/bevy_cli/linter/changelog.html#vXYZ---YYYY-MM-DD
[the migration guide]: https://thebevyflock.github.io/bevy_cli/linter/migration.html#vXYZ-to-vXYZ

> [!WARNING]
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization](https://github.com/bevyengine). Pardon our rough edges, and please consider [submitting an issue](https://github.com/TheBevyFlock/bevy_cli/issues) if you run into trouble!

<!-- You can refer to the compatibility table in `bevy_lint/README.md` for the following two values. -->

This release uses the <!-- `nightly-YYYY-MM-DD` --> toolchain, based on Rust <!-- 1.XX.Y -->, and supports Bevy <!-- X.Y.Z -->. You can install it from Git with the following commands:

<!-- Update `nightly-YYYY-MM-DD` and `lint-vX.Y.Z` in the following code block. -->

```sh
rustup toolchain install nightly-YYYY-MM-DD \
    --component rustc-dev \
    --component llvm-tools

rustup run nightly-YYYY-MM-DD cargo install \
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag lint-vX.Y.Z \
    --locked \
    bevy_lint
```

Alternatively, if you have v0.1.0-alpha.2 or later of the Bevy CLI, you can install the linter with `bevy lint install`:

<!-- Update `vX.Y.Z` in the following code block. -->

```sh
bevy lint install vX.Y.Z
```

<!-- Paste the changelog for this release here. Make sure to include the "All Changes" link. :) -->
````

5. Check the pre-release box if this is an alpha release, then click "Publish release"!
6. Announce the release on Discord! Congrats!

## Post-Release

1. Add a new unreleased section to the top of the changelog (`bevy_lint/CHANGELOG.md`) from the following template:

```markdown
## Unreleased

<!-- Update `lint-vX.Y.Z` in the link to point to the latest release tag. -->

**All Changes**: [`lint-vX.Y.Z...main`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-vX.Y.Z...main)
```

2. Bump the version in `Cargo.toml` to the next `-dev` version, and ensure `Cargo.lock` also updates.
3. Add a new row to the [compatibility table](../../../linter/compatibility.md) for the new `-dev` version.
4. Commit all of these changes and open a pull request.
5. Merge the PR after it has been approved, unblocking frozen pull requests.
