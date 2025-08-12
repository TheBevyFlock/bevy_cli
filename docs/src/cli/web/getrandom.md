# `getrandom`

`getrandom` is a popular crate that allows retrieving random data from system resources.
It is used by `bevy` and many other crates.
It provides multiple backends which retrieve the random data from different sources.

This works great, except when targeting the web.
Unlike most other platforms, there is no compilation target available that guarantees web APIs to exist.
Just because you're building for `wasm32-unknown-unknown` doesn't necessarily mean you are making a web app --
you could also be building a standalone Wasm application.

So the target cannot be used to automatically activate the web backend.
Features are also inadequate: They are additive, so if _any_ dependency would pull in the web feature,
the backend would be used everywhere.
Considering the security-sensitive nature of random data, this was deemed unacceptable.

So in addition to a feature to make the backend available,
`getrandom` requires you to pass a `RUSTFLAG` to the compiler to activate the backend.
This guarantees that the backend can only be configured once (by "outer" package).

## Configuring the web backend

Since you are the application developer, you _know_ that you are building for the web and not just any Wasm target.
This allows you to set up the `getrandom` backend properly, for example like this:

```toml
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
getrandom = { version = "0.3", features = ["wasm_js"] }
getrandom_02 = { version = "0.2", features = ["js"], package = "getrandom" }
```

This activates the necessary feature flags for `getrandom`, accounting for both v0.2 and v0.3 (as they can both be in the dependency tree).

Additionally, you need to add `--cfg getrandom_backend="wasm_js"` to your `RUSTFLAGS`.
This can be done in several places:

- Setting `rustflags` in `.cargo/config.toml`
- Setting `build.rustflags` in `Cargo.toml`
- Setting the `RUSTFLAGS` env variable when running `cargo`

However, not that the rustflags are not merged, but _overwritten_.
So if you have e.g. `rustflags` defined in your `~/.cargo/config.toml` to optimize your compile times,
but then have again `rustflags` defined in your workspace to set the `getrandom` backend,
only the workspace `rustflags` will be used.

## Automated by the CLI

The Bevy CLI automatically applies the web `getrandom` backend when `bevy build web` or `bevy run web` is used
and you haven't configured the backend yourself.
This allows more projects to work out-of-the-box for the web and simplifies the configuration.

You can still explicitly define the backend if you wish (or need to).
