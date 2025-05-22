# Usage

`bevy_lint` has the same API as the `cargo check` command:

```bash
bevy_lint --help
```

If you have the prototype [Bevy CLI](../cli/index.md) installed, the linter is also available through the `lint` subcommand:

```bash
bevy lint --help
```

> **Note**
>
> `bevy_lint` checks your code with the nightly toolchain it was installed with, meaning you _do_ have access to unstable features when it is called. This is best used when [detecting `bevy_lint`](usage/detecting-bevy-lint.md).
