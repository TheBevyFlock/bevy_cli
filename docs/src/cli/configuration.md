# Configuration

The CLI can be configured on a per project basis in the `Cargo.toml` under the `metadata` section.

```toml
[package.metadata.bevy_cli]
```

The CLI supports two targets:

- native: `[package.metadata.bevy_cli.native]`
- web: `[package.metadata.bevy_cli.web]`

For both these targets a release and dev [`profile`] exists that will automatically be chosen by the CLI:

| **Profile Name** | **Configuration Section**                    |
| ---------------- | -------------------------------------------- |
| `release`        | `[package.metadata.bevy_cli.native.release]` |
| `dev`            | `[package.metadata.bevy_cli.native.dev]`     |
| `web-release`    | `[package.metadata.bevy_cli.web.release]`    |
| `web`            | `[package.metadata.bevy_cli.web.dev]`        |

> **Note**
>
> The Web profiles inherits from their native counterpart

## Configuration Merging

Configuration is merged in the following order (from general to specific):

1. Base config: `[package.metadata.bevy_cli]`
2. Profile config `[package.metadata.bevy_cli.{dev|release)]`
3. Target config: `[package.metadata.bevy_cli.{native|web}]`
4. Target + Profile config: `[package.metadata.bevy_cli.{native|web}.{dev|release}]`

## Example

The following `Cargo.toml`

```toml
[package.metadata.bevy_cli]
default-features = true

[package.metadata.bevy_cli.web]
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]

[package.metadata.bevy_cli.web.release]
wasm-opt = true

[package.metadata.bevy_cli.release]
default-features = false
```

When building for web in release mode, the final merged configuration will be:

```toml
# merged from `[package.metadata.bevy_cli.web]`
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]
# merged from `[package.metadata.bevy_cli.web.release]`
wasm-opt = true
# merged from `[package.metadata.bevy_cli.release]`
default-features = false
```

When building for native dev, the final merged configuration will be:

```toml
# merged from [package.metadata.bevy_cli] (default-features are enabled by default if not explicitly turned off)
default-features = true
```

[`profile`]: https://doc.rust-lang.org/cargo/reference/profiles.html
