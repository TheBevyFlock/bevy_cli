# `getrandom`

`getrandom` is a popular crate for retrieving random data from system resources. It provides multiple backends for different platforms, and is an indirect dependency of Bevy.

Usually, `getrandom` is able to automatically select the best backend for the target it is compiled for. This isn't the case when compiled to Wasm, however, because no Wasm target guarantees that JavaScript is available (for the [`Crypto.getRandomValues()`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues) function).

Because of this, `getrandom` cannot automatically activate its web backend, even if the `wasm_js` feature flag is enabled. Instead, [`getrandom` requires the developer to opt-in to the web backend](https://docs.rs/getrandom/0.3.3/getrandom/index.html#webassembly-support) by configuring the `RUSTFLAGS` environmental variable. The Bevy CLI is able to automatically configure `RUSTFLAGS` for you, so you do not need to set it yourself.

## What it does

The CLI will automatically inject `--cfg getrandom_backend="wasm_js"` into `RUSTFLAGS`, opting-in to the JavaScript backend, when:

1. Your dependency tree contains `getrandom`
2. You're building your project in web mode (ex. `bevy build web`)
3. You haven't configured a specific `getrandom` backend in `RUSTFLAGS` already

This simplifies most configuration needed, however you still need to manually enable the `wasm_js` feature flag in your `Cargo.toml`:

```toml
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
getrandom = { version = "0.3", features = ["wasm_js"] }
getrandom_02 = { version = "0.2", features = ["js"], package = "getrandom" }
```

This includes the JavaScript backend when compiling to Wasm for `getrandom` v0.2 and v0.3 (both may be in your dependency tree, depending on what version of Bevy you are using). If you forget to enable the backend, your project will not compile for web and the Bevy CLI will recommend you add the snippet above.
