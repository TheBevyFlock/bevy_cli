# Troubleshooting

## Using with `cranelift`

If you have `cranelift` setup as a custom codegen backend, you may run into the following error when running the linter:

```
error: failed to find a `codegen-backends` folder in the sysroot candidates:
       * ~/.rustup/toolchains/nightly-2025-05-14-x86_64-unknown-linux-gnu
       * ~/.rustup/toolchains/nightly-2025-05-14-x86_64-unknown-linux-gnu
```

This error occurs because you do not have `cranelift` installed for the specific nightly toolchain that the linter uses. You can fix this by installing `rustc-codegen-cranelift-preview` for the linter's toolchain:

```sh
rustup component add rustc-codegen-cranelift-preview --toolchain $TOOLCHAIN_VERSION
```

You can find the value of `$TOOLCHAIN_VERSION` by looking at the [compatibility table](compatibility.md) for the version of the linter you have installed.
