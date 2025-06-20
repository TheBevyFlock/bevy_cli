# Configuration Reference

The following fields exist and can be configured:

- [features](#features)
- [default-features](#default-features)
- [rustflags](#rustflags)
- [wasm-opt](#wasm-opt)

## `features`

- Type: array of strings
- Default: none
- Note: Which [features](https://doc.rust-lang.org/cargo/reference/features.html?highlight=features#the-features-section) to use.

## `default-features`

- Type: boolean
- Default: true
- Note: Whether or not to use the [default-features](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature)

## `rustflags`

- Type: string or array of strings
- Default: none
- Note: Extra command-line flags to pass to rustc. The value may be an array of strings or a space-separated string.

## `wasm-opt`

- Type: boolean or array of strings
- Default: true for web release builds, false for web dev builds and native builds
- Note: Whether or not to use [`wasm-opt`](https://github.com/WebAssembly/binaryen?tab=readme-ov-file#wasm-opt) to optimize the web binary. The specific flags to be used can be passed as array of strings or `true` can be passed to use default options.
