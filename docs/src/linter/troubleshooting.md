# Troubleshooting

## Using with `cranelift`

If you have `cranelift` setup as a custom codegen backend, you may run into the following error when running the linter:

```
error: failed to find a `codegen-backends` folder in the sysroot candidates:
       * ~/.rustup/toolchains/nightly-2025-06-26-x86_64-unknown-linux-gnu
       * ~/.rustup/toolchains/nightly-2025-06-26-x86_64-unknown-linux-gnu
```

This error occurs because you do not have `cranelift` installed for the specific nightly toolchain that the linter uses. You can fix this by installing `rustc-codegen-cranelift-preview` for the linter's toolchain:

```sh
rustup component add rustc-codegen-cranelift-preview --toolchain $TOOLCHAIN_VERSION
```

You can find the value of `$TOOLCHAIN_VERSION` by looking at the [compatibility table](compatibility.md) for the version of the linter you have installed.

## Using with `sccache`

You may run into the following error using `sccache` with `bevy_lint`:

```
error: process didn't exit successfully: `~/.cargo/bin/sccache ~/.cargo/bin/bevy_lint_driver ~/.rustup/toolchains/nightly-2025-04-03-x86_64-unknown-linux-gnu/bin/rustc -vV` (exit status: 2)
--- stderr
sccache: error: failed to execute compile
sccache: caused by: Compiler not supported: "error: expected one of `!` or `[`, found keyword `if`\n --> /tmp/sccacheypo65e/testfile.c:2:2\n  |\n2 | #if defined(__NVCC__) && defined(__NVCOMPILER)\n  |  ^^ expected one of `!` or `[`\n\nerror: aborting due to 1 previous error\n\n"

Check failed: exit status: 101.
Error: command `~/.cargo/bin/bevy_lint ` exited with status code exit status: 101
```

You can fix the error by setting the `CARGO` environmental variable when running the linter. This informs `sccache` that `bevy_lint` uses Cargo:

```sh
CARGO=$(rustup which --toolchain nightly-2025-06-26 cargo) bevy_lint
```

If you use `BEVY_LINT_SYSROOT` instead of Rustup, you can run this instead:

```sh
CARGO="${BEVY_LINT_SYSROOT}/bin/cargo" bevy_lint
```
