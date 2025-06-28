# Environmental Variables

You can configure `bevy_lint`'s behavior using environmental variables. While the default behavior is usually desired, these variables may be useful if you have a non-standard environment.

## `BEVY_LINT_SYSROOT`

By default, the linter assumes you've installed [its required toolchain](compatibility.md) using Rustup. As such, when `BEVY_LINT_SYSROOT` is unset or is empty, the linter will use Rustup to locate the path to the toolchain's system root. (On some platforms, this may be `~/.rustup/toolchains/nightly-YYYY-MM-DD-TARGET`.)

If you do not use Rustup, you must set `BEVY_LINT_SYSROOT` to the path to the toolchain's folder:

```sh
BEVY_LINT_SYSROOT="/path/to/nightly-YYYY-MM-DD-TARGET" bevy_lint
```
