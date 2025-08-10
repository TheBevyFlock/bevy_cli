# Wasm Multi-threading (unstable)

<div class="warning">

**Warning**

This feature is unstable and only available when installing the CLI with the `unstable` feature enabled.
A **nightly Rust** toolchain is also required.

</div>

Did you know that Bevy doesn't natively support multi-threaded web apps?
The Wasm binary runs all on a single thread, leaving a lot of performance on the table.
This can be especially noticeable for audio, resulting in stutters and lag.

The CLI provides an experimental option to build and run apps that use Wasm multi-threading.

## Configuration

You can enable support for Wasm multi-threading in `Cargo.toml`:

```toml
[package.metadata.bevy_cli.unstable]
web-multi-threading = true
```

Alternatively, you can also enable it via the CLI arguments:

```sh
bevy run web --unstable multi-threading
```

## What it Does

To be able to use Wasm multi-threading features, the CLI automatically uses the following options:

- Add `-C target-feature=+atomics,+bulk-memory` to the `RUSTFLAGS` env variable passed to `cargo`. This will enable the Wasm features required for multi-threading.
- Add `-Z build-std=std,panic_abort` to `cargo build` (only if targeting `wasm32-unknown-unknown`, `wasm32v1-none` doesn't need std). This will also enable the Wasm features for std.
- Add the headers `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` to all responses of the web dev server. This enables cross-origin isolation, which is required to use shared memory.

## Publishing a Multi-threaded App

It's important to note that your web server must be configured to use cross-origin isolation in order to use Wasm multi-threading.
This is a security feature to prevent Spectre-like attacks.

In particular, the server needs to set the `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` headers.

When using **itch.io**, you can do so by [enabling `SharedArrayBuffer` support](https://itch.io/t/2025776/experimental-sharedarraybuffer-support).
Go to **Embed Options** > **Frame Options** > enable `SharedArrayBuffer` support.
