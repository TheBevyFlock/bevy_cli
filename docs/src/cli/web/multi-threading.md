# Wasm Multi-Threading (Unstable)

<div class="warning">

**Warning**

This feature is unstable and only available when installing the CLI with the `unstable` feature (enabled by default). Expect to encounter more bugs and a worse user interface until this is stabilized.  A **nightly Rust toolchain** is also required!

</div>

Crates such as `firewheel-web-audio` and `bevy_seedling` (through its `web-audio` feature) can take advantage of multi-threading on Wasm. This can be especially beneficial for audio on the web, but requires special flags and setup in order to use. The Bevy CLI can simplify this process for you, making it easier to setup multi-threading for the web.

Note that this **does not enable Bevy's multi-threaded scheduler**. The Bevy engine does not yet take advantage of multi-threading on the web, only certain 3rd-party crates do.

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
This is a security feature to prevent [Spectre](https://meltdownattack.com/)-like attacks.

In particular, the server needs to set the `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` headers.

When using **itch.io**, you can do so by [enabling `SharedArrayBuffer` support](https://itch.io/t/2025776/experimental-sharedarraybuffer-support).
Go to **Embed Options** > **Frame Options** > enable `SharedArrayBuffer` support.
