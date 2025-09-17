# Configuration Reference

The following fields exist and can be configured:

- [Configuration Reference](#configuration-reference)
  - [`features`](#features)
  - [`default-features`](#default-features)
  - [`target`](#target)
  - [`rustflags`](#rustflags)
  - [`wasm-opt`](#wasm-opt)
  - [`headers`](#headers)
  - [`unstable`](#unstable)
    - [`unstable.web-multi-threading`](#unstableweb-multi-threading)

## `features`

- Type: array of strings
- Default: none
- Note: Which [features](https://doc.rust-lang.org/cargo/reference/features.html?highlight=features#the-features-section) to use.

## `default-features`

- Type: boolean
- Default: true
- Note: Whether or not to use the [default-features](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature).

## `target`

- Type: string
- Default: your host target triple for native builds, `wasm32-unknown-unknown` for web builds
- Note: To get a list of supported targets, run `rustc --print target-list`.

## `rustflags`

- Type: string or array of strings
- Default: none
- Note: Extra command-line flags to pass to rustc. The value may be an array of strings or a space-separated string.

## `wasm-opt`

- Type: boolean or array of strings
- Default: true for web `--release` builds, false for web development builds and native builds
- Note: Whether or not to use [`wasm-opt`](https://github.com/WebAssembly/binaryen?tab=readme-ov-file#wasm-opt) to optimize the web binary. The specific flags to be used can be passed as array of strings or `true` can be passed to use default options (`--strip-debug` and `-Os`).

## `headers`

- Type: array of strings
- Default: none
- Note: Headers may be in the format of `KEY:VALUE` or `KEY=VALUE`. These headers are appended to the defaults set by the HTTP server. For a list of supported values, please see [MDN's docs on HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers).

## `unstable`

- Type: map
- Note: Enable unstable CLI features. Only available when building the CLI with the `unstable` feature (enabled by default).

### `unstable.web-multi-threading`

- Type: boolean
- Default: false
- Note: Enable Wasm multi-threading features.
