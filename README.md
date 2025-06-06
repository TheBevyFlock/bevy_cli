# Bevy CLI (Alpha)

A prototype [Bevy] CLI tool intended to streamline common tasks when working on projects. Please see the [initial scope document] and [original issue] for history and motivation.

- [**Documentation**](https://thebevyflock.github.io/bevy_cli/)
- [**Repository**](https://github.com/TheBevyFlock/bevy_cli)
- [**Issue Tracker**](https://github.com/TheBevyFlock/bevy_cli/issues)

If you need assistance or want to help, reach out to the [`bevy_cli` working group channel] in the [Bevy Discord].

[Bevy]: https://bevyengine.org
[initial scope document]: https://hackmd.io/cCHAfbtaSviU_MDnbNHKxg
[original issue]: https://github.com/bevyengine/bevy/issues/436
[`bevy_cli` working group channel]: https://discord.com/channels/691052431525675048/1278871953721262090
[Bevy Discord]: https://discord.gg/bevy

## Installation

<!-- Please keep this section synchronized with the `mdbook` docs. -->

As the CLI is currently an unofficial tool, it is not yet published to <https://crates.io>. It is available [on Github](https://github.com/TheBevyFlock/bevy_cli), however.

You may compile the latest version of the CLI from scratch using `cargo install`:

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-v0.1.0-alpha.1 --locked bevy_cli
```

<details>
    <summary><strong>Precompiled Binaries</strong></summary>

The CLI is precompiled for Linux, Windows, and macOS. You may install the latest precompiled binary using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```sh
cargo binstall --git https://github.com/TheBevyFlock/bevy_cli --version v0.1.0-alpha.1 --locked bevy_cli
```

You can manually download the precompiled binaries from the [release page](https://github.com/TheBevyFlock/bevy_cli/releases).

</details>

### Bleeding Edge

> **Here be dragons! 🐉**
>
> You may run into bugs when using the unstable version of the CLI. You've been warned, and have fun! :)

If you want to try out the newest unstable features, you may install the CLI from the [`main`](https://github.com/TheBevyFlock/bevy_cli/tree/main) branch:

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --branch main --locked bevy_cli
```

## Quick Start

<!-- Please keep this section synchronized with the `mdbook` docs. -->

With the following steps, you can create a new 2D app with Bevy and run it in your browser:

1. Create a new Bevy app using [the 2D template](https://github.com/TheBevyFlock/bevy_new_2d):

    ```sh
    bevy new -t=2d my_bevy_app
    ```

2. Navigate into the folder:

   ```sh
   cd my_bevy_app
   ```

3. Check the code quality with the linter:

    ```sh
    bevy lint
    ```

4. Run the app in the browser:

    ```sh
    bevy run web --open
    ```

## License

The Bevy CLI is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for more information!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
